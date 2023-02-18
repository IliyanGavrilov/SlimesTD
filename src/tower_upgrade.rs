use bevy::utils::HashMap;
use bevy::prelude::*;
use crate::{TowerType};

pub struct TowerUpgradePlugin;

impl Plugin for TowerUpgradePlugin {
  fn build(&self, app: &mut App) {
    app
      // .add_system_set(SystemSet::on_update(GameState::Gameplay)
      //   .with_system(tower_click)
      //   .with_system(tower_ui_interaction))
      .add_startup_system_to_stage(StartupStage::PreStartup, load_upgrades);
  }
}

#[derive(Resource)]
pub struct Upgrades {
  pub upgrades: HashMap<TowerType, Vec<Vec<Upgrade>>>
}

#[derive(Component, Reflect, FromReflect, Clone)]
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

#[derive(Component, Reflect, FromReflect, Clone)]
pub struct Upgrade {
  pub upgrade: HashMap<TowerStat, i32>,
  pub price: usize
}

#[derive(Hash, Eq, PartialEq, Reflect, FromReflect, Clone)]
pub enum TowerStat { // Projectile speed, pierce !!!
  Damage,
  AttackSpeed,
  Range
}

fn load_upgrades(
  mut commands: Commands
) {
  commands.insert_resource(Upgrades {
    upgrades: HashMap::from([(
      TowerType::Nature, vec![vec![
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 1), (TowerStat::Range, 50)]),
          price: 50
        },
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 3), (TowerStat::AttackSpeed, 95)]),
          price: 200
        },
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 10), (TowerStat::AttackSpeed, 10)]),
          price: 300
        }]]), (
      TowerType::Fire, vec![vec![
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 1), (TowerStat::Range, 50)]),
          price: 50
        },
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 3), (TowerStat::AttackSpeed, 10)]),
          price: 200
        },
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 10), (TowerStat::AttackSpeed, 10)]),
          price: 300
        }]]), (
      TowerType::Ice, vec![vec![
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 1), (TowerStat::Range, 50)]),
          price: 50
        },
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 3), (TowerStat::AttackSpeed, 10)]),
          price: 200
        },
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 10), (TowerStat::AttackSpeed, 10)]),
          price: 300
        }]]), (
      TowerType::Dark, vec![vec![
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 1), (TowerStat::Range, 50)]),
          price: 50
        },
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 3), (TowerStat::AttackSpeed, 10)]),
          price: 200
        },
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 10), (TowerStat::AttackSpeed, 10)]),
          price: 300
        }]]), (
      TowerType::Mage, vec![vec![
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 1), (TowerStat::Range, 50)]),
          price: 50
        },
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 3), (TowerStat::AttackSpeed, 10)]),
          price: 200
        },
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 10), (TowerStat::AttackSpeed, 10)]),
          price: 300
        }]]), (
      TowerType::Archmage, vec![vec![
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 1), (TowerStat::Range, 50)]),
          price: 50
        },
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 3), (TowerStat::AttackSpeed, 10)]),
          price: 200
        },
        Upgrade {
          upgrade: HashMap::from([(TowerStat::Damage, 10), (TowerStat::AttackSpeed, 10)]),
          price: 300
        }]]),
    ])
  });
}