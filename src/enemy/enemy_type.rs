use bevy::prelude::*;
use bevy::utils::HashMap;
use strum_macros::{Display};
use serde::{Serialize, Deserialize};
use std::fs::File;

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
  Red
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct EnemyTypeStats {
  pub enemy: HashMap<EnemyType, EnemyBundle>
}

pub fn load_enemy_type_stats(
  mut commands: Commands
) {
  let f = File::open("./assets/game_data/enemy_stats.ron")
    .expect("Failed opening enemy stats file!");
  let enemy_type_stats: EnemyTypeStats = match ron::de::from_reader(f) {
    Ok(x) => x,
    Err(e) => {
      info!("Failed to load upgrades: {}", e);
  
      std::process::exit(1);
    }
  };
  
  commands.insert_resource(enemy_type_stats);
}

impl EnemyType {
  pub fn get_enemy(
    &self,
    map_path: &MapPath,
    path: Path,
    enemy_stats: &EnemyTypeStats
  ) -> EnemyBundle {
    let direction = map_path.checkpoints[path.index + 1];
    
    let mut enemy_bundle = enemy_stats.enemy[self].clone();
    
    enemy_bundle.path = path;
    enemy_bundle.movement.direction = direction;
    
    return enemy_bundle;
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
      EnemyType::Red => TextureAtlasSprite::new(70)
    };
  
    return SpriteSheetBundle {
      texture_atlas: assets.enemy.clone(),
      transform: Transform::from_translation(position),
      sprite: texture_atlas_sprite,
      ..default()
    };
  }
}