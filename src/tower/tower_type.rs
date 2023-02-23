use bevy::prelude::*;
use bevy::utils::HashMap;
use strum_macros::{EnumIter, Display};
use serde::{Serialize, Deserialize};
use std::fs::File;

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

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct TowerTypeStats {
  pub tower: HashMap<TowerType, TowerBundle>
}

pub fn load_tower_type_stats(
  mut commands: Commands
) {
  let f = File::open("./assets/game_data/tower_stats.ron")
    .expect("Failed opening tower stats file!");
  let tower_type_stats: TowerTypeStats = match ron::de::from_reader(f) {
    Ok(x) => x,
    Err(e) => {
      info!("Failed to load upgrades: {}", e);
      
      std::process::exit(1);
    }
  };
  
  commands.insert_resource(tower_type_stats);
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
          movement: Movement {
            direction: Vec3::new(0.00000001,0.,0.),
            speed: 1500.
          },
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
        movement: Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
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
        movement: Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
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
        movement: Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
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
        movement: Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
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
        movement: Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
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