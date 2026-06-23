use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::assets::*;
use crate::enemy::*;
use crate::movement::*;
use crate::{FloatingTextEvent, GameDifficulty, GameState, Map, game_not_paused};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Enemy>()
      .register_type::<Path>()
      .add_event::<EnemyDeathEvent>()
      .add_system(scale_hp_for_difficulty.in_set(OnUpdate(GameState::Gameplay)))
      .add_system(despawn_enemy_on_death.in_set(OnUpdate(GameState::Gameplay)).run_if(game_not_paused))
      .add_system(tick_slowed.in_set(OnUpdate(GameState::Gameplay)).run_if(game_not_paused))
      .add_system(tick_poison.in_set(OnUpdate(GameState::Gameplay)).run_if(game_not_paused))
      .add_system(apply_knockback.in_set(OnUpdate(GameState::Gameplay)).run_if(game_not_paused))
      .add_system(cleanup_enemies.in_schedule(OnExit(GameState::Gameplay)));
  }
}

pub struct EnemyDeathEvent {
  pub transform: Transform,
  pub atlas: Handle<TextureAtlas>,
  pub sprite_index: usize,
  pub flip_x: bool,
}

#[derive(Bundle, Debug, Serialize, Deserialize, Clone)]
pub struct EnemyBundle {
  pub enemy_type: EnemyType,
  pub enemy: Enemy,
  pub movement: Movement,
  pub animation_indices: AnimationIndices,
  pub animation_timer: AnimationTimer,
  pub path: Path,
  pub name: Name,
}

impl Default for EnemyBundle {
  fn default() -> Self {
    Self {
      enemy_type: EnemyType::Green,
      enemy: Enemy::new(1),
      movement: Movement {
        speed: 50.,
        ..default()
      },
      animation_indices: AnimationIndices { first: 0, last: 9 },
      animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
      path: Path { index: 0 },
      name: Name::new("GreenEnemy"),
    }
  }
}

#[derive(Reflect, Debug, Component, Default, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Enemy {
  pub health: i32,
}

#[derive(Reflect, Component, Default, Clone, Serialize, Debug, Deserialize)]
#[reflect(Component)]
pub struct Path {
  pub index: usize,
}

impl Enemy {
  pub fn new(health: i32) -> Self {
    Self { health }
  }
}

/// A temporary movement-speed status applied by Slow/Stun on-hit effects.
/// `factor` multiplies the enemy's speed (0.0 = stunned, 0.5 = half speed).
#[derive(Component)]
pub struct Slowed {
  pub factor: f32,
  pub timer: Timer,
}

fn tick_slowed(mut commands: Commands, time: Res<Time>, mut slowed: Query<(Entity, &mut Slowed)>) {
  for (entity, mut slow) in &mut slowed {
    slow.timer.tick(time.delta());
    if slow.timer.finished() {
      commands.entity(entity).remove::<Slowed>();
    }
  }
}

/// Damage-over-time status applied by the Poison on-hit effect.
#[derive(Component)]
pub struct Poisoned {
  pub dps: u32,
  /// Overall lifetime of the poison.
  pub duration: Timer,
  /// Cadence at which a damage tick is applied.
  pub tick: Timer,
}

impl Poisoned {
  pub fn new(dps: u32, duration_secs: f32) -> Self {
    Self {
      dps,
      duration: Timer::from_seconds(duration_secs, TimerMode::Once),
      tick: Timer::from_seconds(1.0, TimerMode::Repeating),
    }
  }
}

/// Brief backwards shove applied by the Knockback on-hit effect.
#[derive(Component)]
pub struct KnockedBack {
  pub strength: f32,
  pub timer: Timer,
}

impl KnockedBack {
  pub fn new(strength: f32) -> Self {
    Self {
      strength,
      timer: Timer::from_seconds(0.15, TimerMode::Once),
    }
  }
}

fn apply_knockback(
  mut commands: Commands,
  time: Res<Time>,
  mut knocked: Query<(Entity, &mut Transform, &Movement, &mut KnockedBack)>,
) {
  for (entity, mut transform, movement, mut knock) in &mut knocked {
    knock.timer.tick(time.delta());
    // Push opposite to the enemy's heading (back along its path).
    if movement.direction.length_squared() > 0.0 {
      let back = -movement.direction.normalize();
      transform.translation += back * knock.strength * time.delta_seconds();
    }
    if knock.timer.finished() {
      commands.entity(entity).remove::<KnockedBack>();
    }
  }
}

fn tick_poison(
  mut commands: Commands,
  time: Res<Time>,
  mut poisoned: Query<(Entity, &mut Enemy, &Transform, &mut Poisoned)>,
  mut float_writer: EventWriter<FloatingTextEvent>,
) {
  for (entity, mut enemy, transform, mut poison) in &mut poisoned {
    poison.duration.tick(time.delta());
    poison.tick.tick(time.delta());

    if poison.tick.just_finished() && enemy.health > 0 {
      enemy.health -= poison.dps as i32;
      float_writer.send(FloatingTextEvent {
        position: transform.translation,
        text: format!("-{}", poison.dps),
        color: Color::GREEN,
      });
    }

    if poison.duration.finished() {
      commands.entity(entity).remove::<Poisoned>();
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
  enemy_stats: &EnemyTypeStats,
) {
  commands
    .spawn(enemy_type.get_enemy(map_path, path, enemy_stats))
    .insert(enemy_type.get_sprite_sheet_bundle(assets, position));
}

fn scale_hp_for_difficulty(
  difficulty: Res<GameDifficulty>,
  mut new_enemies: Query<&mut Enemy, Added<Enemy>>,
) {
  if *difficulty == GameDifficulty::Test {
    for mut enemy in &mut new_enemies {
      enemy.health = 1;
    }
  }
}

fn cleanup_enemies(mut commands: Commands, enemies: Query<Entity, With<Enemy>>) {
  for entity in &enemies {
    commands.entity(entity).despawn_recursive();
  }
}

fn despawn_enemy_on_death(
  mut commands: Commands,
  enemies: Query<(
    Entity,
    &Enemy,
    &Transform,
    &TextureAtlasSprite,
    &Handle<TextureAtlas>,
  )>,
  mut death_event_writer: EventWriter<EnemyDeathEvent>,
) {
  for (entity, enemy, transform, sprite, atlas) in &enemies {
    if enemy.health <= 0 {
      death_event_writer.send(EnemyDeathEvent {
        transform: *transform,
        atlas: atlas.clone(),
        sprite_index: sprite.index,
        flip_x: sprite.flip_x,
      });
      commands.entity(entity).despawn_recursive();
    }
  }
}