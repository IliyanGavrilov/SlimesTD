use bevy::prelude::*;
use crate::{Bullet, GameAssets, Movement, Tower, TowerBundle};
use strum_macros::{EnumIter, Display};
use bevy_inspector_egui::Inspectable;

#[derive(EnumIter, Inspectable, Component, Display, Clone, Copy, Debug, PartialEq)]
pub enum TowerType {
  Nature,
  Fire,
  Ice,
  Dark,
  Mage,
  Archmage
}

impl TowerType {
  pub fn get_tower(&self, assets: &GameAssets, position: Vec3) -> TowerBundle {
    match self {
      TowerType::Nature => TowerBundle {
          tower_type: TowerType::Nature,
          tower: Tower::new(
            Vec3::new(20., 0., 0.),
            1,
            Timer::from_seconds(1., TimerMode::Repeating),
            10,
            100
          ),
          sprite: SpriteBundle {
            texture: assets.wizard_nature.clone(),
            transform: Transform::from_translation(position),
            ..default()
          },
          name: Name::new("NatureTower")
      },
      TowerType::Fire => TowerBundle {
        tower_type: TowerType::Fire,
        tower: Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        sprite: SpriteBundle {
          texture: assets.wizard_fire.clone(),
          transform: Transform::from_translation(position),
          ..default()
        },
        name: Name::new("FireTower")
      },
      TowerType::Ice => TowerBundle {
        tower_type: TowerType::Ice,
        tower: Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        sprite: SpriteBundle {
          texture: assets.wizard_ice.clone(),
          transform: Transform::from_translation(position),
          ..default()
        },
        name: Name::new("IceTower")
      },
      TowerType::Dark => TowerBundle {
        tower_type: TowerType::Dark,
        tower: Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        sprite: SpriteBundle {
          texture: assets.wizard_dark.clone(),
          transform: Transform::from_translation(position),
          ..default()
        },
        name: Name::new("DarkTower")
      },
      TowerType::Mage => TowerBundle {
        tower_type: TowerType::Mage,
        tower: Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        sprite: SpriteBundle {
          texture: assets.wizard_mage.clone(),
          transform: Transform::from_translation(position),
          ..default()
        },
        name: Name::new("MageTower")
      },
      TowerType::Archmage => TowerBundle {
        tower_type: TowerType::Archmage,
        tower: Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        sprite: SpriteBundle {
          texture: assets.wizard_archmage.clone(),
          transform: Transform::from_translation(position),
          ..default()
        },
        name: Name::new("ArchmageTower")
      },
    }
  }
  
  pub fn get_bullet(&self, damage: i32, assets: &GameAssets) -> (Bullet, Movement, Handle<Image>) {
    match self {
      TowerType::Nature => (
        Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        assets.wizard_nature_bullet.clone()
      ),
      TowerType::Fire => (
        Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        assets.wizard_fire_bullet.clone()
      ),
      TowerType::Ice => (
        Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        assets.wizard_ice_bullet.clone()
      ),
      TowerType::Dark => (
        Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        assets.wizard_dark_bullet.clone()
      ),
      TowerType::Mage => (
        Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        assets.wizard_mage_bullet.clone()
      ),
      TowerType::Archmage => (
        Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        assets.wizard_archmage_bullet.clone()
      )
    }
  }
  
  pub fn path(&self) -> &str {
    match self {
      TowerType::Nature => "tower_buttons/wizard_nature_button.png",
      TowerType::Fire => "tower_buttons/wizard_fire_button.png",
      TowerType::Ice => "tower_buttons/wizard_ice_button.png",
      TowerType::Dark => "tower_buttons/wizard_dark_button.png",
      TowerType::Mage => "tower_buttons/wizard_mage_button.png",
      TowerType::Archmage => "tower_buttons/wizard_archmage_button.png",
      
    }
  }
}