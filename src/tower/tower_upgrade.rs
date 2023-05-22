use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

use crate::tower::*;

#[derive(Resource, Deserialize, TypeUuid, Debug)]
#[uuid = "34ef287b-4806-41da-a102-fc9effcb280f"]
pub struct Upgrades {
  pub upgrades: HashMap<TowerType, Vec<Vec<Upgrade>>>,
}

#[derive(Component, Reflect, FromReflect, Clone, Serialize, Deserialize)]
pub struct TowerUpgrades {
  pub upgrades: Vec<usize>,
}

impl Default for TowerUpgrades {
  fn default() -> Self {
    Self {
      upgrades: vec![0, 0, 0],
    }
  }
}

#[derive(Component, Reflect, FromReflect, Clone, Deserialize, Debug)]
pub struct Upgrade {
  pub upgrade: HashMap<TowerStat, i32>,
  pub cost: usize,
}

#[derive(Hash, Eq, PartialEq, Reflect, FromReflect, Clone, Deserialize, Debug)]
pub enum TowerStat {
  // Projectile speed, pierce !!!
  Damage,
  AttackSpeed,
  Range,
}
