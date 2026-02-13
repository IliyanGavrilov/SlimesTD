use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::assets::*;
use crate::enemy::*;
use crate::map::*;

#[derive(Component, Display, Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum EnemyType {
  Green,
  Yellow,
  Pink,
  White,
  Blue,
  Orange,
  Purple,
  Red,
}

#[derive(Resource, Debug, Serialize, Deserialize, TypeUuid, Clone)]
#[uuid = "7aad646e-4054-44d7-b138-1fb79f73f9c1"]
pub struct EnemyTypeStats {
  pub enemy: HashMap<EnemyType, EnemyBundle>,
}

impl EnemyType {
  pub fn get_enemy(&self, map_path: &Map, path: Path, enemy_stats: &EnemyTypeStats) -> EnemyBundle {
    let direction = map_path.checkpoints[path.index + 1];

    let mut enemy_bundle = enemy_stats.enemy[self].clone();

    enemy_bundle.path = path;
    enemy_bundle.movement.direction = direction;

    enemy_bundle
  }

  pub fn get_sprite_sheet_bundle(&self, assets: &GameAssets, position: Vec3) -> SpriteSheetBundle {
    let texture_atlas_sprite = match self {
      EnemyType::Green => TextureAtlasSprite::new(0),
      EnemyType::Yellow => TextureAtlasSprite::new(10),
      EnemyType::Pink => TextureAtlasSprite::new(20),
      EnemyType::White => TextureAtlasSprite::new(30),
      EnemyType::Blue => TextureAtlasSprite::new(40),
      EnemyType::Orange => TextureAtlasSprite::new(50),
      EnemyType::Purple => TextureAtlasSprite::new(60),
      EnemyType::Red => TextureAtlasSprite::new(70),
    };

    SpriteSheetBundle {
      texture_atlas: assets.enemy.clone(),
      transform: Transform::from_translation(position),
      sprite: texture_atlas_sprite,
      ..default()
    }
  }
}

#[cfg(test)]
#[path = "enemy/type_tests.rs"]
mod tests;