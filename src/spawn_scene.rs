use bevy::prelude::*;
use crate::{enemy::spawn_enemy, EnemyType, GameAssets, tower::spawn_tower, TowerType};

pub struct SpawnScenePlugin;

impl Plugin for SpawnScenePlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(spawn_basic_scene)
       .add_startup_system(spawn_camera);
  }
}

fn spawn_basic_scene(
  mut commands: Commands,
  assets: Res<GameAssets> // Tower and enemy assets
) {
  // Enemy
  spawn_enemy(&mut commands,
              EnemyType::Green,
                &assets,
              Vec3::new(-200., 0., 0.),
              Vec3::new(-200., 9999999., 0.));
  
  // Enemy 2
  spawn_enemy(&mut commands,
              EnemyType::Yellow,
              &assets,
              Vec3::new(-200., -100., 0.),
              Vec3::new(-200., 9999999., 0.));
  
  // Tower
  spawn_tower(&mut commands,
              TowerType::Fire,
              &assets,
              Vec3::new(100., 0., 0.));
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}