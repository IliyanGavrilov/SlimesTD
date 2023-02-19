use bevy::prelude::*;

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
  commands.spawn((Camera2dBundle::default(), MainCamera));
}