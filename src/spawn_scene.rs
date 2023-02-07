use bevy::prelude::*;
use crate::{enemy::spawn_enemy, EnemyType, GameAssets, tower::spawn_tower, TowerType};

pub struct SpawnScenePlugin;

impl Plugin for SpawnScenePlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(spawn_basic_scene)
       .add_startup_system(spawn_camera);
  }
}

use bevy::sprite::MaterialMesh2dBundle;

fn spawn_basic_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  assets: Res<GameAssets> // Tower and enemy assets
) {
  // Enemy
  spawn_enemy(&mut commands,
              EnemyType::Red,
                &assets,
              Vec3::new(0., 0., 0.),
              Vec3::new(0., 9999999., 0.));
  
  // Enemy 2
  spawn_enemy(&mut commands,
              EnemyType::Purple,
              &assets,
              Vec3::new(0., -100., 0.),
              Vec3::new(0., 9999999., 0.));
  
  // Tower
  spawn_tower(&mut commands,
              TowerType::Fire,
              &assets,
              Vec3::new(100., 0., 0.));
  
  // Tower range
  commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(shape::Circle::new(125.).into()).into(),
    material: materials.add(ColorMaterial::from(
      Color::rgba_u8(0, 0, 0, 85))),
    transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
    ..default()
  }).insert(Name::new("TowerRangeCircle"));
  
  // Tower range when trying to place on path/invalid tile
  // commands.spawn(MaterialMesh2dBundle {
  //   mesh: meshes.add(shape::Circle::new(125.).into()).into(),
  //   material: materials.add(ColorMaterial::from(
  //     Color::rgba_u8(202, 0, 0, 150))),
  //   transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
  //   ..default()
  // }).insert(Name::new("TowerRangeCircle"));
}

// Main camera marker component
#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
  commands.spawn((Camera2dBundle::default(), MainCamera));
}