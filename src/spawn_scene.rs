use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct SpawnScenePlugin;

impl Plugin for SpawnScenePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(spawn_camera);
  }
}

// Main camera marker component
#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
  let mut camera = Camera2dBundle::default(); // !!!
  camera.projection.scaling_mode = ScalingMode::Auto { min_width: 1472., min_height: 828.0 };
  commands.spawn((camera, MainCamera));
}