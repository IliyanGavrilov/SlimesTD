use bevy::prelude::*;
use bevy::utils::HashMap;
use strum_macros::{EnumIter, Display};
use serde::{Serialize, Deserialize};
use bevy::reflect::TypeUuid;

use crate::assets::*;
use crate::tower::*;
use crate::movement::*;

#[derive(EnumIter, Component, Display, Clone, Copy,
Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TowerType {
  Nature,
  Fire,
  Ice,
  Dark,
  Mage,
  Archmage
}

#[derive(Resource, Serialize, Deserialize, Clone, TypeUuid)]
#[uuid = "410719fd-234e-4e88-8549-4ff3004041a9"]
pub struct TowerTypeStats {
  pub tower: HashMap<TowerType, TowerBundle>
}

impl TowerType {
  pub fn get_tower(&self, tower_stats: &TowerTypeStats) -> TowerBundle {
    return tower_stats.tower[self].clone();
  }
  
  pub fn get_sprite_sheet_bundle(&self, assets: &GameAssets, position: Vec3) -> SpriteBundle {
    let texture = match self {
      TowerType::Nature => assets.wizard_nature.clone(),
      TowerType::Fire => assets.wizard_fire.clone(),
      TowerType::Ice => assets.wizard_ice.clone(),
      TowerType::Dark => assets.wizard_dark.clone(),
      TowerType::Mage => assets.wizard_mage.clone(),
      TowerType::Archmage => assets.wizard_archmage.clone()
    };
    
    return SpriteBundle {
      texture,
      transform: Transform::from_translation(position),
      ..default()
    }
  }
  
  pub fn get_bullet(&self, damage: u32, assets: &GameAssets, position: Transform) -> BulletBundle {
    match self {
      TowerType::Nature => BulletBundle {
          bullet: Bullet {
            damage,
            lifetime: Timer::from_seconds(1.25, TimerMode::Once),
          },
          movement: Movement::new(Vec3::new(0.00000001, 0., 0.), 1500.),
          sprite: SpriteBundle {
            texture: assets.wizard_nature_bullet.clone(),
            transform: position,
            ..default()
          },
          name: Name::new("Bullet")
      },
      TowerType::Fire => BulletBundle {
        bullet: Bullet {
          damage,
          lifetime: Timer::from_seconds(1.25, TimerMode::Once),
        },
        movement: Movement::new(Vec3::new(0.00000001, 0., 0.), 1500.),
        sprite: SpriteBundle {
          texture: assets.wizard_fire_bullet.clone(),
          transform: position,
          ..default()
        },
        name: Name::new("Bullet")
      },
      TowerType::Ice => BulletBundle {
        bullet: Bullet {
          damage,
          lifetime: Timer::from_seconds(1.25, TimerMode::Once),
        },
        movement: Movement::new(Vec3::new(0.00000001, 0., 0.), 1500.),
        sprite: SpriteBundle {
          texture: assets.wizard_ice_bullet.clone(),
          transform: position,
          ..default()
        },
        name: Name::new("Bullet")
      },
      TowerType::Dark => BulletBundle {
        bullet: Bullet {
          damage,
          lifetime: Timer::from_seconds(1.25, TimerMode::Once),
        },
        movement: Movement::new(Vec3::new(0.00000001, 0., 0.), 1500.),
        sprite: SpriteBundle {
          texture: assets.wizard_dark_bullet.clone(),
          transform: position,
          ..default()
        },
        name: Name::new("Bullet")
      },
      TowerType::Mage => BulletBundle {
        bullet: Bullet {
          damage,
          lifetime: Timer::from_seconds(1.25, TimerMode::Once),
        },
        movement: Movement::new(Vec3::new(0.00000001, 0., 0.), 1500.),
        sprite: SpriteBundle {
          texture: assets.wizard_mage_bullet.clone(),
          transform: position,
          ..default()
        },
        name: Name::new("Bullet")
      },
      TowerType::Archmage => BulletBundle {
        bullet: Bullet {
          damage,
          lifetime: Timer::from_seconds(1.25, TimerMode::Once),
        },
        movement: Movement::new(Vec3::new(0.00000001, 0., 0.), 1500.),
        sprite: SpriteBundle {
          texture: assets.wizard_archmage_bullet.clone(),
          transform: position,
          ..default()
        },
        name: Name::new("Bullet")
      }
    }
  }
}