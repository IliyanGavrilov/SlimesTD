use bevy::prelude::*;
use crate::{Base, enemy::spawn_enemy, EnemyType, GameAssets, GameState, MapPath, Path, tower::spawn_tower, TowerType};

pub struct SpawnScenePlugin;

impl Plugin for SpawnScenePlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Base>()
      .add_system_set(SystemSet::on_enter(GameState::Gameplay)
        .with_system(spawn_basic_scene))
      .add_startup_system(spawn_camera);
  }
}

fn spawn_basic_scene(
  mut commands: Commands,
  assets: Res<GameAssets>, // Tower and enemy assets
  map_path: Res<MapPath>
) {
  commands.spawn(Base {health: 100}).insert(Name::new("Base"));
  // Enemy
  spawn_enemy(&mut commands,
              &map_path,
              EnemyType::Red,
              &assets,
              map_path.checkpoints[0],
              Path {index: 0});
  
  // Enemy 2
  spawn_enemy(&mut commands,
              &map_path,
              EnemyType::Purple,
              &assets,
              map_path.checkpoints[0],
              Path {index: 0});
  
  // Tower
  spawn_tower(&mut commands,
              TowerType::Fire,
              &assets,
              Vec3::new(100., 0., 0.));
  
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