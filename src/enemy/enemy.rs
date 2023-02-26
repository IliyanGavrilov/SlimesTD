use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::assets::*;
use crate::enemy::*;
use crate::movement::*;
use crate::{GameState, Map};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Enemy>()
      .register_type::<Path>()
      .add_event::<EnemyDeathEvent>()
      .add_startup_system(load_enemy_type_stats)
      .add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(despawn_enemy_on_death));
  }
}

pub struct EnemyDeathEvent;

#[derive(Bundle, Serialize, Deserialize, Clone)]
pub struct EnemyBundle {
  pub enemy_type: EnemyType,
  pub enemy: Enemy,
  pub movement: Movement,
  pub animation_indices: AnimationIndices,
  pub animation_timer: AnimationTimer,
  pub path: Path,
  pub name: Name
}

impl Default for EnemyBundle {
  fn default() -> Self {
    Self {
      enemy_type: EnemyType::Green,
      enemy: Enemy::new(1),
      movement: Movement { speed: 50. , ..default()},
      animation_indices: AnimationIndices {first: 0, last: 9},
      animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
      path: Path {index: 0},
      name: Name::new("GreenEnemy")
    }
  }
}

#[derive(Reflect, Component, Default, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Enemy {
  pub health: i32
}

#[derive(Reflect, Component, Default, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Path {
  pub index: usize
}

impl Enemy {
  pub fn new(health: i32) -> Self {
    Self {
      health
    }
  }
}

pub fn spawn_enemy(
  commands: &mut Commands,
  map_path: &Map,
  enemy_type: EnemyType,
  assets: &GameAssets,
  position: Vec3,
  path: Path,
  enemy_stats: &EnemyTypeStats
) {
  commands.spawn(enemy_type.get_enemy(map_path, path, enemy_stats))
    .insert(enemy_type.get_sprite_sheet_bundle(assets, position));
}

// !!! Spawn weaker enemy?
fn despawn_enemy_on_death(
  mut commands: Commands,
  enemies: Query<(Entity, &mut Enemy)>,
  mut death_event_writer: EventWriter<EnemyDeathEvent>
) {
  for (entity, enemy) in &enemies {
    if enemy.health <= 0 {
      death_event_writer.send(EnemyDeathEvent);
      commands.entity(entity).despawn_recursive();
    }
  }
}