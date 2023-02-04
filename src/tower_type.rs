use bevy::prelude::*;
use crate::{Bullet, GameAssets, Movement, Tower};
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
  pub fn get_tower(&self, assets: &GameAssets) -> (Tower, Handle<Image>) {
    match self {
      TowerType::Nature => (
        Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        assets.wizard_nature.clone()
      ),
      TowerType::Fire => (
        Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        assets.wizard_fire.clone()
      ),
      TowerType::Ice => (
        Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        assets.wizard_ice.clone()
      ),
      TowerType::Dark => (
        Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        assets.wizard_dark.clone()
      ),
      TowerType::Mage => (
        Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        assets.wizard_mage.clone()
      ),
      TowerType::Archmage => (
        Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        assets.wizard_archmage.clone()
      )
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