use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use strum_macros::{Display};
use crate::{AnimationIndices, Enemy, EnemyBundle, GameAssets, MapPath, Movement, Path};

#[derive(Inspectable, Component, Display, Clone, Copy, Debug, PartialEq)]
pub enum EnemyType {
  Green,
  Yellow,
  Pink,
  White,
  Blue,
  Orange,
  Purple,
  Red
}

impl EnemyType {
  pub fn get_enemy(&self, assets: &GameAssets, map_path: &MapPath, position: Vec3, path: Path) -> EnemyBundle {
    let direction = map_path.checkpoints[1];
    match self {
      EnemyType::Green => EnemyBundle {
        movement: Movement { direction, speed: 15. },
        sprite_sheet_bundle: SpriteSheetBundle {
          texture_atlas: assets.enemy.clone(),
          transform: Transform::from_translation(position),
          sprite: TextureAtlasSprite::new(0),
          ..default()
        },
        path,
        ..default()
      },
      EnemyType::Yellow => EnemyBundle {
        enemy_type: EnemyType::Yellow,
        enemy: Enemy::new(2),
        movement: Movement { direction, speed: 15. },
        animation_indices: AnimationIndices {first: 10, last: 19},
        sprite_sheet_bundle: SpriteSheetBundle {
          texture_atlas: assets.enemy.clone(),
          transform: Transform::from_translation(position),
          sprite: TextureAtlasSprite::new(10),
          ..default()
        },
        path,
        name: Name::new("YellowEnemy"),
        ..default()
      },
      EnemyType::Pink => EnemyBundle {
        enemy_type: EnemyType::Pink,
        enemy: Enemy::new(3),
        movement: Movement { direction, speed: 15. },
        animation_indices: AnimationIndices {first: 20, last: 29},
        sprite_sheet_bundle: SpriteSheetBundle {
          texture_atlas: assets.enemy.clone(),
          transform: Transform::from_translation(position),
          sprite: TextureAtlasSprite::new(20),
          ..default()
        },
        path,
        name: Name::new("PinkEnemy"),
        ..default()
      },
      EnemyType::White => EnemyBundle {
        enemy_type: EnemyType::White,
        enemy: Enemy::new(4),
        movement: Movement { direction, speed: 15. },
        animation_indices: AnimationIndices {first: 30, last: 39},
        sprite_sheet_bundle: SpriteSheetBundle {
          texture_atlas: assets.enemy.clone(),
          transform: Transform::from_translation(position),
          sprite: TextureAtlasSprite::new(30),
          ..default()
        },
        path,
        name: Name::new("WhiteEnemy"),
        ..default()
      },
      EnemyType::Blue => EnemyBundle {
        enemy_type: EnemyType::Blue,
        enemy: Enemy::new(5),
        movement: Movement { direction, speed: 15. },
        animation_indices: AnimationIndices {first: 40, last: 49},
        sprite_sheet_bundle: SpriteSheetBundle {
          texture_atlas: assets.enemy.clone(),
          transform: Transform::from_translation(position),
          sprite: TextureAtlasSprite::new(40),
          ..default()
        },
        path,
        name: Name::new("BlueEnemy"),
        ..default()
      },
      EnemyType::Orange => EnemyBundle {
        enemy_type: EnemyType::Orange,
        enemy: Enemy::new(6),
        movement: Movement { direction, speed: 15. },
        animation_indices: AnimationIndices {first: 50, last: 59},
        sprite_sheet_bundle: SpriteSheetBundle {
          texture_atlas: assets.enemy.clone(),
          transform: Transform::from_translation(position),
          sprite: TextureAtlasSprite::new(50),
          ..default()
        },
        path,
        name: Name::new("OrangeEnemy"),
        ..default()
      },
      EnemyType::Purple => EnemyBundle {
        enemy_type: EnemyType::Purple,
        enemy: Enemy::new(7),
        movement: Movement { direction, speed: 15. },
        animation_indices: AnimationIndices {first: 60, last: 69},
        sprite_sheet_bundle: SpriteSheetBundle {
          texture_atlas: assets.enemy.clone(),
          transform: Transform::from_translation(position),
          sprite: TextureAtlasSprite::new(60),
          ..default()
        },
        path,
        name: Name::new("PurpleEnemy"),
        ..default()
      },
      EnemyType::Red => EnemyBundle {
        enemy_type: EnemyType::Green,
        enemy: Enemy::new(8),
        movement: Movement { direction, speed: 15. },
        animation_indices: AnimationIndices {first: 70, last: 79},
        sprite_sheet_bundle: SpriteSheetBundle {
          texture_atlas: assets.enemy.clone(),
          transform: Transform::from_translation(position),
          sprite: TextureAtlasSprite::new(70),
          ..default()
        },
        path,
        name: Name::new("RedEnemy"),
        ..default()
      }
    }
  }
}