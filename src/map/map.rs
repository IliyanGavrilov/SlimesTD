use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::camera::ScalingMode;
use serde::{Deserialize, Serialize};

use crate::gameplay_ui::*;
use crate::movement::*;
use crate::{Enemy, GameAssets, GameData, GameState, Path};

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        (load_map, setup_camera.after(load_map)).in_schedule(OnExit(GameState::AssetLoading)),
      )
      .add_system(render_map.in_schedule(OnEnter(GameState::Gameplay)))
      .add_systems((update_enemy_checkpoint, despawn_enemy).in_set(OnUpdate(GameState::Gameplay)));
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

#[derive(Resource, Serialize, Deserialize, TypeUuid)]
#[uuid = "58d181c2-39f7-4ac7-8ae7-b3cee0667ce2"]
pub struct Map {
  pub width: usize,
  pub height: usize,
  pub tiles: Vec<Vec<Tile>>,
  pub tile_size: usize,
  pub checkpoints: Vec<Vec3>,
}

fn load_map(game_data: Res<GameData>, mut map: ResMut<Assets<Map>>) {
  let Some(map) = map.get_mut(&game_data.map)
    else { return; };

  let mut path_tiles = vec![];
  let mut spawn: Point = Default::default();
  let mut end: Point = Default::default();

  map.tiles.reverse();

  for (y, row) in map.tiles.iter().enumerate() {
    for (x, tile) in row.iter().enumerate() {
      match tile {
        Tile::Spawn => spawn = Point { x, y },
        Tile::Path(_) => path_tiles.push(Point { x, y }),
        Tile::End => end = Point { x, y },
        _ => {}
      }
    }
  }

  map.checkpoints.push(spawn.to_vec3());
  map.create_checkpoints(path_tiles, spawn, end);
}

impl Map {
  fn create_checkpoints(&mut self, mut path_tiles: Vec<Point>, spawn: Point, end: Point) {
    self.checkpoints.push(spawn.to_coordinate(self.tile_size, false).to_vec3());

    let mut last_point = spawn;

    while let Some(next_idx) = path_tiles.iter().position(|p| last_point.is_adjacent_to(*p)) {
      let next_point = path_tiles.remove(next_idx);
      self.checkpoints.push(next_point.to_coordinate(self.tile_size, true).to_vec3());
      last_point = next_point;
    }

    self.checkpoints.push(end.to_coordinate(self.tile_size, true).to_vec3());
  }
}

// Main camera marker component
#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands, game_data: Res<GameData>, map: Res<Assets<Map>>) {
  let Some(map) = map.get(&game_data.map)
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
  game_data: Res<GameData>,
  map: Res<Assets<Map>>,
  assets: Res<GameAssets>,
) {
  let Some(map) = map.get(&game_data.map)
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Tile {
  Grass,
  Water,
  Spawn,
  Path(Vec<usize>),
  End,
  Empty,
}

#[derive(Component)]
pub struct MapTile {
  pub coordinate: Point,
  pub tile: Tile,
}

fn despawn_enemy(
  mut commands: Commands,
  mut enemies: Query<(Entity, &Enemy, &mut Path)>,
  mut base: Query<&mut Base>,
  game_data: Res<GameData>,
  map: Res<Assets<Map>>,
) {
  let Some(map) = map.get(&game_data.map)
    else { return; };

  let mut base = base.single_mut();

  for (entity, enemy, path) in &mut enemies {
    if path.index >= map.checkpoints.len() {
      damage_base(&mut commands, &entity, enemy.health, &mut base);
    }
  }
}

fn update_enemy_checkpoint(
  mut enemies: Query<(&mut Movement, &mut Transform, &mut Path)>,
  game_data: Res<GameData>,
  map: Res<Assets<Map>>,
  time: Res<Time>,
) {
  let Some(map) = map.get(&game_data.map)
    else { return; };

  for (mut movement, mut transform, mut path) in &mut enemies {
    if path.index >= map.checkpoints.len() {
      continue;
    }

    let distance = map.checkpoints[path.index] - transform.translation;
    if distance == Vec3::ZERO {
      path.index += 1;
      continue;
    }
    let enemy_movement = distance.normalize() * movement.speed * time.delta_seconds();

    if enemy_movement.length() > distance.length() {
      transform.translation = map.checkpoints[path.index];
      movement.distance_travelled += distance.length();
      movement.direction = map.checkpoints[path.index] - transform.translation;
      path.index += 1;
    } else {
      movement.distance_travelled += enemy_movement.length();
      movement.direction = map.checkpoints[path.index] - transform.translation;
      transform.translation += enemy_movement;
    }
  }
}
