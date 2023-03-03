use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use std::fs::File;
use serde::{Serialize, Deserialize};

use crate::gameplay_ui::*;
use crate::{Enemy, GameAssets, GameState, Path};
use crate::movement::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app.add_system_set(SystemSet::on_enter(GameState::Gameplay)
      .with_system(render_map))
      .add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(update_enemy_checkpoint)
        .with_system(despawn_enemy))
      .add_startup_system_to_stage(StartupStage::PreStartup, load_map)
      .add_startup_system(setup_camera);
  }
}

#[derive(Resource)]
pub struct MapPath {
  pub checkpoints: Vec<Vec3>,
}

#[derive(Component)]
pub struct TileMap;

#[derive(Default, Debug)]
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
}

#[derive(Resource, Serialize, Deserialize)]
pub struct Map {
  pub width: usize,
  pub height: usize,
  pub tiles: Vec<Vec<Tile>>,
  pub tile_size: usize,
  pub checkpoints: Vec<Vec3>
}

fn load_map(mut commands: Commands) {
  let f = File::open("./assets/data/map.ron").expect("Failed opening map file!");
  let mut map: Map = match ron::de::from_reader(f) {
    Ok(x) => x,
    Err(e) => {
      panic!("Failed to load map: {}", e);
    }
  };
  
  let mut path_tiles = vec![];
  let mut spawn: Point = Default::default();
  let mut end: Point = Default::default();
  
  map.tiles.reverse();
  
  for (y, row) in map.tiles.iter().enumerate() {
    for (x, tile) in row.iter().enumerate() {
      match tile {
        Tile::Spawn => spawn = Point {x, y},
        Tile::Path(_) => path_tiles.push(Point {x, y}),
        Tile::End => end = Point {x, y},
        _ => {}
      }
    }
  }
  
  map.checkpoints.push(spawn.to_vec3());
  map.create_checkpoints(path_tiles, spawn, end);
  commands.insert_resource(map);
}

impl Map {
  fn create_checkpoints(&mut self, mut path_tiles: Vec<Point>, spawn: Point, end: Point) {
    let mut last_point = spawn;
    
    loop {
      let next_point_position = path_tiles.iter().position(|point| {
        let length = Vec2::new(
          last_point.x as f32 - point.x as f32,
          last_point.y as f32 - point.y as f32,
        ).length();
        length >= 0.9 && length <= 1.1
      });
      if let Some(next_point_position) = next_point_position {
        let next_point = path_tiles.remove(next_point_position);
        self.checkpoints.push(Coordinate {
          x: (next_point.x * self.tile_size) as f32,
          y: (next_point.y * self.tile_size + self.tile_size / 2) as f32,
        }.to_vec3());
        last_point = next_point;
      }
      else {
        self.checkpoints.push(Coordinate {
          x: (end.x * self.tile_size) as f32,
          y: (end.y * self.tile_size + self.tile_size / 2) as f32,
        }.to_vec3());
        self.checkpoints[0] = Coordinate {
          x: self.checkpoints[0].x * self.tile_size as f32,
          y: self.checkpoints[0].y * self.tile_size as f32,
        }.to_vec3();
        
        return;
      }
    }
  }
}

// Main camera marker component
#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands, map: Res<Map>) {
  let mut camera = Camera2dBundle::default();
  camera.transform.translation.x = (map.width as f32 / 2. - 0.5) * map.tile_size as f32;
  camera.transform.translation.y = (map.height as f32 / 2. - 0.5) * map.tile_size as f32;
  camera.projection.scaling_mode = ScalingMode::Auto { min_width: 1280., min_height: 720.0 };
  commands.spawn((camera, MainCamera));
}

fn render_map(
  mut commands: Commands,
  map: Res<Map>, assets: Res<GameAssets>
) {
  
  commands.spawn(SpatialBundle::default()).with_children(|commands| {
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
          coordinate: Point {x: column, y: row},
          tile: tile.clone(),
        })
        .insert(Name::new(format!("{:?}", tile)));
    }
  }})
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
  Empty
}

#[derive(Component)]
pub struct MapTile {
  pub coordinate: Point,
  pub tile: Tile
}

fn despawn_enemy(
  mut commands: Commands,
  mut enemies: Query<(Entity, &Enemy, &mut Path)>,
  mut base: Query<&mut Base>,
  map: Res<Map>
) {
  let mut base = base.single_mut();
  
  for (entity,
    enemy,
    path) in &mut enemies {
    if path.index >= map.checkpoints.len() {
      damage_base(&mut commands, &entity, enemy.health, &mut base);
    }
  }
}

fn update_enemy_checkpoint(
  mut enemies: Query<(&mut Movement, &mut Transform, &mut Path)>,
  map: Res<Map>,
  time: Res<Time>
) {
  for (mut movement,
    mut transform,
    mut path) in &mut enemies {
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
    }
    else {
      movement.distance_travelled += enemy_movement.length();
      movement.direction = map.checkpoints[path.index] - transform.translation;
      transform.translation += enemy_movement;
    }
  }
}