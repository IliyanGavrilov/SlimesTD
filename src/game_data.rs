use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{EnemyTypeStats, Map, TowerTypeStats, Upgrades, Waves};

#[derive(Resource)]
struct EnemyTypeStatsHandle(Handle<EnemyTypeStats>);

#[derive(AssetCollection, Resource)]
pub struct GameData {
  #[asset(path = "data/stats.enemy_types.ron")]
  pub enemy_type_stats: Handle<EnemyTypeStats>,
  #[asset(path = "data/level1.map.ron")]
  pub map: Handle<Map>,
  #[asset(path = "data/stats.tower_stats.ron")]
  pub tower_type_stats: Handle<TowerTypeStats>,
  #[asset(path = "data/tower.upgrades.ron")]
  pub tower_upgrades: Handle<Upgrades>,
  #[asset(path = "data/enemy.waves.ron")]
  pub enemy_waves: Handle<Waves>,
}