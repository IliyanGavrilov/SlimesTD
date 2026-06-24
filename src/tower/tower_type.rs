use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

use crate::assets::*;
use crate::movement::*;
use crate::tower::*;

#[derive(
  EnumIter, Component, Display, Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum TowerType {
  Nature,
  Fire,
  Ice,
  Dark,
  Mage,
  Archmage,
  /// Earns gold passively every 15 seconds.
  FarmPassive,
  /// Earns gold for every enemy killed on the map by any tower.
  FarmKill,
  /// Earns a large bonus when each wave is cleared.
  FarmWave,
  /// Shoots enemies and earns gold for every enemy it personally kills.
  FarmSelfKill,
}

#[derive(Resource, Serialize, Deserialize, Clone, TypeUuid)]
#[uuid = "410719fd-234e-4e88-8549-4ff3004041a9"]
pub struct TowerTypeStats {
  pub tower: HashMap<TowerType, TowerBundle>,
}

impl TowerType {
  pub fn description(&self) -> &'static str {
    match self {
      TowerType::Nature      => "Poisons enemies over time",
      TowerType::Fire        => "Splash damage to nearby enemies",
      TowerType::Ice         => "Rapid-fire, slows enemies it hits",
      TowerType::Dark        => "Knocks back; sees invisible enemies",
      TowerType::Mage        => "Hits hard and briefly stuns",
      TowerType::Archmage    => "Chains lightning between enemies",
      TowerType::FarmPassive => "Earns gold passively over time",
      TowerType::FarmKill    => "Gold for any kill on the map",
      TowerType::FarmWave    => "Big bonus gold each wave cleared",
      TowerType::FarmSelfKill=> "Shoots; earns gold per own kill",
    }
  }

  pub fn get_tower(&self, tower_stats: &TowerTypeStats) -> TowerBundle {
    tower_stats.tower[self].clone()
  }

  pub fn get_farm_tower(&self) -> Option<FarmTower> {
    match self {
      TowerType::FarmPassive => Some(FarmTower::new(FarmBehavior::Passive {
        income: 50,
        timer: Timer::from_seconds(15.0, TimerMode::Repeating),
      })),
      TowerType::FarmKill => Some(FarmTower::new(FarmBehavior::Kill { income_per_kill: 5 })),
      TowerType::FarmWave => Some(FarmTower::new(FarmBehavior::Wave { income_per_wave: 75 })),
      TowerType::FarmSelfKill => Some(FarmTower::new(FarmBehavior::SelfKill { income_per_kill: 3 })),
      _ => None,
    }
  }

  pub fn sprite_scale(&self) -> f32 {
    match self {
      TowerType::FarmSelfKill => 1.5,
      TowerType::FarmWave     => 0.75,
      _                       => 1.0,
    }
  }

  pub fn placement_radius(&self) -> f32 {
    self.sprite_scale() * 20.0
  }

  pub fn get_sprite_sheet_bundle(&self, assets: &GameAssets, position: Vec3) -> SpriteBundle {
    let texture = match self {
      TowerType::Nature => assets.wizard_nature.clone(),
      TowerType::Fire => assets.wizard_fire.clone(),
      TowerType::Ice => assets.wizard_ice.clone(),
      TowerType::Dark => assets.wizard_dark.clone(),
      TowerType::Mage => assets.wizard_mage.clone(),
      TowerType::Archmage => assets.wizard_archmage.clone(),
      TowerType::FarmPassive | TowerType::FarmKill | TowerType::FarmWave | TowerType::FarmSelfKill => {
        assets.wizard_nature.clone()
      }
    };

    SpriteBundle {
      texture,
      transform: Transform::from_translation(position)
        .with_scale(Vec3::splat(self.sprite_scale())),
      ..default()
    }
  }

  pub fn get_bullet(
    &self,
    damage: u32,
    pierce: u32,
    projectile_speed: f32,
    effect: Option<OnHitEffect>,
    assets: &GameAssets,
    position: Transform,
  ) -> BulletBundle {
    let texture = match self {
      TowerType::Nature => assets.wizard_nature_bullet.clone(),
      TowerType::Fire => assets.wizard_fire_bullet.clone(),
      TowerType::Ice => assets.wizard_ice_bullet.clone(),
      TowerType::Dark => assets.wizard_dark_bullet.clone(),
      TowerType::Mage => assets.wizard_mage_bullet.clone(),
      TowerType::Archmage => assets.wizard_archmage_bullet.clone(),
      TowerType::FarmSelfKill => assets.wizard_nature_bullet.clone(),
      TowerType::FarmPassive | TowerType::FarmKill | TowerType::FarmWave => {
        unreachable!("Passive farm towers do not shoot")
      }
    };
    BulletBundle {
      bullet: Bullet {
        damage,
        pierce_remaining: pierce,
        lifetime: Timer::from_seconds(1.25, TimerMode::Once),
        effect,
      },
      movement: Movement::new(Vec3::new(0.00000001, 0., 0.), projectile_speed),
      sprite: SpriteBundle {
        texture,
        transform: position,
        ..default()
      },
      name: Name::new("Bullet"),
    }
  }
}