use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::camera::ScalingMode;
use serde::{Deserialize, Serialize};

use crate::gameplay_ui::*;
use crate::movement::*;
use crate::{
  BaseDamagedEvent, Enemy, GameAssets, GameData, GameState, Path, SelectedMap, Slowed,
  game_not_paused,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(setup_camera.in_schedule(OnExit(GameState::AssetLoading)))
      .add_systems(
        (initialize_selected_map, render_map.after(initialize_selected_map))
          .in_schedule(OnEnter(GameState::Gameplay)),
      )
      .add_systems(
        (
          update_enemy_checkpoint.run_if(game_not_paused),
          despawn_enemy.run_if(game_not_paused),
        )
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_system(cleanup_map.in_schedule(OnExit(GameState::Gameplay)));
  }
}

#[derive(Resource)]
pub struct MapPath {
  pub checkpoints: Vec<Vec3>,
}

#[derive(Component)]
pub struct TileMap;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Point {
  pub x: usize,
  pub y: usize,
}

impl Point {
  pub fn to_vec3(&self) -> Vec3 {
    Vec3::new(self.x as f32, self.y as f32, 0.)
  }

  pub fn to_coordinate(self, tile_size: usize, center_y: bool) -> Coordinate {
    let y_offset = if center_y { tile_size / 2 } else { 0 };
    Coordinate {
      x: (self.x * tile_size) as f32,
      y: (self.y * tile_size + y_offset) as f32,
    }
  }

  pub fn is_adjacent_to(self, other: Point) -> bool {
    let distance = Vec2::new(
      self.x as f32 - other.x as f32,
      self.y as f32 - other.y as f32,
    ).length();
    (0.9..=1.1).contains(&distance)
  }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Coordinate {
  pub x: f32,
  pub y: f32,
}

impl Coordinate {
  pub fn to_vec3(&self) -> Vec3 {
    Vec3::new(self.x, self.y, 0.)
  }
}

#[derive(Resource, Serialize, Deserialize, TypeUuid, Default)]
#[uuid = "58d181c2-39f7-4ac7-8ae7-b3cee0667ce2"]
pub struct Map {
  pub width: usize,
  pub height: usize,
  pub tiles: Vec<Vec<Tile>>,
  pub tile_size: usize,
  pub checkpoints: Vec<Vec3>,
  // World-space checkpoints for each lane, built at init (route 0 = main path)
  #[serde(skip)]
  pub route_paths: Vec<Vec<Vec3>>,
  // Tracks whether tiles have been Y-flipped for Bevy's coordinate system
  #[serde(default)]
  pub initialized: bool,
}

fn initialize_selected_map(selected_map: Res<SelectedMap>, mut maps: ResMut<Assets<Map>>) {
  let Some(map) = maps.get_mut(&selected_map.0) else { return; };
  if map.initialized {
    return;
  }

  map.tiles.reverse();
  map.initialized = true;

  let mut spawn: Point = Default::default();
  let mut end: Point = Default::default();
  // Path tiles grouped by lane (lane 0 = `Path`, lane n = `Lane(n, _)`).
  let mut lanes: Vec<Vec<Point>> = vec![vec![]];

  for (y, row) in map.tiles.iter().enumerate() {
    for (x, tile) in row.iter().enumerate() {
      let point = Point { x, y };
      match tile {
        Tile::Spawn => spawn = point,
        Tile::End => end = point,
        Tile::Path(_) => lanes[0].push(point),
        Tile::Lane(lane_ids, _) => {
          for &lane in lane_ids {
            if lane >= lanes.len() {
              lanes.resize(lane + 1, vec![]);
            }
            lanes[lane].push(point);
          }
        }
        _ => {}
      }
    }
  }

  // Lane 0 stays the canonical `checkpoints`; extra lanes build their own route.
  map.create_checkpoints(lanes[0].clone(), spawn, end);
  let mut route_paths = vec![map.checkpoints.clone()];
  for tiles in lanes.into_iter().skip(1) {
    route_paths.push(map.build_route(tiles, spawn, end));
  }
  map.route_paths = route_paths;
}

impl Map {
  fn create_checkpoints(&mut self, path_tiles: Vec<Point>, spawn: Point, end: Point) {
    self.checkpoints = self.build_route(path_tiles, spawn, end);
  }

  /// The order list of a path tile, whichever lane it belongs to.
  fn orders_at(&self, p: Point) -> Option<&Vec<usize>> {
    match &self.tiles[p.y][p.x] {
      Tile::Path(orders) | Tile::Lane(_, orders) => Some(orders),
      _ => None,
    }
  }

  /// Builds one lane's world-space checkpoints: spawn (offset off-screen), the
  /// path tiles in order, then the end. Tiles carrying an order > 0 use numbered
  /// pathfinding (loops/crossings); otherwise simple adjacency walking.
  fn build_route(&self, path_tiles: Vec<Point>, spawn: Point, end: Point) -> Vec<Vec3> {
    let offset_distance = (self.tile_size * 2) as f32;
    let mut spawn_coord = spawn.to_coordinate(self.tile_size, false);
    if spawn.y == 0 {
      spawn_coord.y -= offset_distance;
    } else if spawn.y == self.height - 1 {
      spawn_coord.y += offset_distance;
    } else if spawn.x == 0 {
      spawn_coord.x -= offset_distance;
    } else if spawn.x == self.width - 1 {
      spawn_coord.x += offset_distance;
    }

    let mut checkpoints = vec![spawn_coord.to_vec3()];

    let has_numbered_path = path_tiles
      .iter()
      .any(|&p| self.orders_at(p).is_some_and(|o| o.iter().any(|&order| order > 0)));

    if has_numbered_path {
      let max_order = path_tiles
        .iter()
        .filter_map(|&p| self.orders_at(p).and_then(|o| o.iter().max().copied()))
        .max()
        .unwrap_or(0);

      for i in 0..=max_order {
        if let Some(&point) = path_tiles
          .iter()
          .find(|&&p| self.orders_at(p).is_some_and(|o| o.contains(&i)))
        {
          checkpoints.push(point.to_coordinate(self.tile_size, true).to_vec3());
        }
      }
    } else {
      let mut remaining_tiles = path_tiles;
      let mut last_point = spawn;
      while let Some(next_idx) = remaining_tiles.iter().position(|p| last_point.is_adjacent_to(*p)) {
        let next_point = remaining_tiles.remove(next_idx);
        checkpoints.push(next_point.to_coordinate(self.tile_size, true).to_vec3());
        last_point = next_point;
      }
    }

    checkpoints.push(end.to_coordinate(self.tile_size, true).to_vec3());
    checkpoints
  }

  /// Checkpoints for the given lane, falling back to route 0 / the main path.
  pub fn route(&self, route: usize) -> &Vec<Vec3> {
    self.route_paths.get(route).unwrap_or(&self.checkpoints)
  }

  /// How many lanes this map has (always at least 1).
  pub fn route_count(&self) -> usize {
    self.route_paths.len().max(1)
  }
}

/// Marker for the game's main 2D camera.
#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands, game_data: Res<GameData>, maps: Res<Assets<Map>>) {
  // Use map1 for camera dimensions (both maps are the same size)
  let Some(map) = maps.get(&game_data.map)
    else { return; };
  let mut camera = Camera2dBundle::default();
  camera.transform.translation.x = (map.width as f32 / 2. - 0.5) * map.tile_size as f32;
  camera.transform.translation.y = (map.height as f32 / 2. - 0.5) * map.tile_size as f32;
  camera.projection.scaling_mode = ScalingMode::AutoMin {
    min_width: 1280.,
    min_height: 720.0,
  };
  commands.spawn((camera, MainCamera));
}

fn render_map(
  mut commands: Commands,
  selected_map: Res<SelectedMap>,
  maps: Res<Assets<Map>>,
  assets: Res<GameAssets>,
) {
  let Some(map) = maps.get(&selected_map.0)
    else { return; };

  commands
    .spawn(SpatialBundle::default())
    .with_children(|commands| {
      for row in 0..map.height {
        for column in 0..map.width {
          let tile = &map.tiles[row][column];
          commands
            .spawn(SpriteBundle {
              texture: assets.get_tile(tile).clone(),
              transform: Transform::from_translation(Vec3::new(
                column as f32 * map.tile_size as f32,
                row as f32 * map.tile_size as f32,
                -0.000000000000001,
              )),
              ..Default::default()
            })
            .insert(MapTile {
              coordinate: Point { x: column, y: row },
              tile: tile.clone(),
            })
            .insert(Name::new(format!("{:?}", tile)));
        }
      }
    })
    .insert(TileMap)
    .insert(Name::new("TileMap"));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerrainType {
  Grass,
  Water,
  // Add future terrain types here (e.g. Mountain, Desert)
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Tile {
  Grass,
  Water,
  Spawn,
  Path(Vec<usize>),
  // A path tile shared by the listed lanes (lane 0 is also `Path`). Lets a map
  // split into several routes and share segments, e.g. a common entry corridor
  // (see `Map::route`). The second vec is the tile ordering, like `Path`.
  Lane(Vec<usize>, Vec<usize>),
  End,
  Empty,
}

impl Tile {
  pub fn terrain_type(&self) -> Option<TerrainType> {
    match self {
      Tile::Grass => Some(TerrainType::Grass),
      Tile::Water => Some(TerrainType::Water),
      _ => None,
    }
  }
}

#[derive(Component)]
pub struct MapTile {
  pub coordinate: Point,
  pub tile: Tile,
}

fn cleanup_map(mut commands: Commands, tilemaps: Query<Entity, With<TileMap>>) {
  for entity in &tilemaps {
    commands.entity(entity).despawn_recursive();
  }
}

fn despawn_enemy(
  mut commands: Commands,
  mut enemies: Query<(Entity, &Enemy, &mut Path)>,
  mut base: Query<&mut Base>,
  selected_map: Res<SelectedMap>,
  maps: Res<Assets<Map>>,
  mut base_damaged: EventWriter<BaseDamagedEvent>,
) {
  let Some(map) = maps.get(&selected_map.0)
    else { return; };

  let mut base = base.single_mut();

  for (entity, enemy, path) in &mut enemies {
    if path.index >= map.route(path.route).len() {
      damage_base(&mut commands, &entity, enemy.health, &mut base);
      base_damaged.send(BaseDamagedEvent);
    }
  }
}

fn update_enemy_checkpoint(
  mut enemies: Query<(&mut Movement, &mut Transform, &mut Path, Option<&Slowed>)>,
  selected_map: Res<SelectedMap>,
  maps: Res<Assets<Map>>,
  time: Res<Time>,
) {
  let Some(map) = maps.get(&selected_map.0)
    else { return; };

  for (mut movement, mut transform, mut path, slowed) in &mut enemies {
    let checkpoints = map.route(path.route);
    if path.index >= checkpoints.len() {
      continue;
    }

    let distance = checkpoints[path.index] - transform.translation;
    if distance == Vec3::ZERO {
      path.index += 1;
      continue;
    }
    // Slow/Stun scales the effective speed without touching the base value.
    let speed = movement.speed * slowed.map_or(1.0, |s| s.factor);
    let enemy_movement = distance.normalize() * speed * time.delta_seconds();

    if enemy_movement.length() > distance.length() {
      transform.translation = checkpoints[path.index];
      movement.distance_travelled += distance.length();
      movement.direction = checkpoints[path.index] - transform.translation;
      path.index += 1;
    } else {
      movement.distance_travelled += enemy_movement.length();
      movement.direction = checkpoints[path.index] - transform.translation;
      transform.translation += enemy_movement;
    }
  }
}

mod tests;