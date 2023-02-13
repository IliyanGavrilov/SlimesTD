use bevy::prelude::*;

pub struct SpawnScenePlugin;

impl Plugin for SpawnScenePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(spawn_camera);
  }
}

// !!!
// Tower range when trying to place on path/invalid tile
// commands.spawn(MaterialMesh2dBundle {
//   mesh: meshes.add(shape::Circle::new(125.).into()).into(),
//   material: materials.add(ColorMaterial::from(
//     Color::rgba_u8(202, 0, 0, 150))),
//   transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
//   ..default()
// }).insert(Name::new("TowerRangeCircle"));

// Main camera marker component
#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
  commands.spawn((Camera2dBundle::default(), MainCamera));
}