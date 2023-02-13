use std::time::Duration;
use bevy::prelude::*;
use bevy::time::Stopwatch;
pub use crate::{GameAssets, Movement, enemy_type::EnemyType};
use crate::{AnimationIndices, AnimationTimer, GameState, MapPath, WaveIndex};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Enemy>()
      .register_type::<Path>()
      .add_event::<EnemyDeathEvent>()
      .add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(despawn_enemy_on_death)
        .with_system(tick_enemy_time_alive));
  }
}

pub struct EnemyDeathEvent;

#[derive(Bundle)]
pub struct EnemyBundle {
  pub enemy_type: EnemyType,
  pub enemy: Enemy,
  pub movement: Movement,
  pub animation_indices: AnimationIndices,
  pub animation_timer: AnimationTimer,
  pub sprite_sheet_bundle: SpriteSheetBundle,
  pub path: Path,
  pub time_alive: TimeAlive,
  pub wave_index: WaveIndex,
  pub name: Name
}

impl Default for EnemyBundle {
  fn default() -> Self {
    EnemyBundle {
      enemy_type: EnemyType::Green,
      enemy: Enemy::new(1),
      movement: Movement { direction: default(), speed: 15. },
      animation_indices: AnimationIndices {first: 0, last: 9},
      animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
      sprite_sheet_bundle: SpriteSheetBundle::default(),
      path: Path {index: 0},
      time_alive: TimeAlive {time_alive: Stopwatch::new()},
      wave_index: WaveIndex {index: 0},
      name: Name::new("GreenEnemy")
    }
  }
}

// !!! Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy {
  pub health: u32
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Path {
  pub index: usize
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TimeAlive {
  pub time_alive: Stopwatch
}

impl Enemy {
  pub fn new(health: u32) -> Self {
    Self {
      health
    }
  }
}

fn tick_enemy_time_alive(
  mut enemies: Query<&mut TimeAlive>,
  time: Res<Time>
) {
  for mut enemy in &mut enemies {
    enemy.time_alive.tick(time.delta());
  }
}

pub fn spawn_enemy(
  commands: &mut Commands,
  map_path: &MapPath,
  enemy_type: EnemyType,
  assets: &GameAssets,
  position: Vec3,
  path: Path,
  wave_index: WaveIndex
) {
  commands.spawn(enemy_type.get_enemy(assets, map_path, position, path, wave_index));
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