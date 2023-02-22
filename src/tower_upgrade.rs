use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::File;
use crate::{TowerType};

pub struct TowerUpgradePlugin;

impl Plugin for TowerUpgradePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system_to_stage(StartupStage::PreStartup, load_upgrades);
  }
}

#[derive(Resource, Deserialize, Debug)]
pub struct Upgrades {
  pub upgrades: HashMap<TowerType, Vec<Vec<Upgrade>>>
}

#[derive(Component, Reflect, FromReflect, Clone, Serialize, Deserialize)]
pub struct TowerUpgrades {
  pub upgrades: Vec<usize>
}

impl Default for TowerUpgrades {
  fn default() -> Self {
    Self {
      upgrades: vec![0, 0, 0]
    }
  }
}

#[derive(Component, Reflect, FromReflect, Clone, Deserialize, Debug)]
pub struct Upgrade {
  pub upgrade: HashMap<TowerStat, i32>,
  pub cost: usize
}

#[derive(Hash, Eq, PartialEq, Reflect, FromReflect, Clone, Deserialize, Debug)]
pub enum TowerStat { // Projectile speed, pierce !!!
  Damage,
  AttackSpeed,
  Range
}

fn load_upgrades(
  mut commands: Commands
) {
  let f = File::open("./assets/game_data/upgrades.ron").expect("Failed opening upgrades file!");
  let upgrades: Upgrades = match ron::de::from_reader(f) {
    Ok(x) => x,
    Err(e) => {
      info!("Failed to load upgrades: {}", e);
      
      std::process::exit(1);
    }
  };
  
  commands.insert_resource(upgrades);
}