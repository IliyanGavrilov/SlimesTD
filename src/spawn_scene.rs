use bevy::prelude::*;
use crate::{AnimationIndices, AnimationTimer, Enemy, GameAssets, Movement, TargetingPriority, Tower};

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
  commands.spawn(SpriteSheetBundle {
    texture_atlas: assets.enemy.clone(),
    transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
    ..default()
  })
    .insert(AnimationIndices {first: 0, last: 9})
    // Animate slime jumping
    .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
    .insert(Enemy {
      health: 5,
    })
    .insert(Movement {
      direction: Vec3::new(-200., 9999999., 0.),
      speed: 15.
    })
    .insert(Name::new("Enemy")); // !!! Debug
  
  // Enemy 2
  commands.spawn(SpriteSheetBundle {
    texture_atlas: assets.enemy.clone(),
    transform: Transform::from_translation(Vec3::new(-200., -100., 0.)),
    sprite: TextureAtlasSprite::new(10),
    ..default()
  })
    .insert(AnimationIndices {first: 10, last: 19})
    // Animate slime jumping
    .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
    .insert(Enemy {
      health: 5,
    })
    .insert(Movement {
      direction: Vec3::new(-200., 999999., 0.),
      speed: 15.
    })
    .insert(Name::new("Enemy 2")); // !!! Debug
  
  // Tower
  commands.spawn(SpriteBundle {
    texture: assets.tower.clone(),
    transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
    sprite: Sprite {
      ..default()
    },
    ..default()
  })
    .insert(Tower {
      bullet_spawn_offset: Vec3::new(15., 0., 0.),
      damage: 1,
      attack_speed: Timer::from_seconds(1., TimerMode::Repeating),
      range: 10,
      price: 100,
      sell_price: (100/3) as i32,
      target: TargetingPriority::CLOSE
      //..default()
    })
    .insert(Name::new("Tower")); // !!! Debug
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}