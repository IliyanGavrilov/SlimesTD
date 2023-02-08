use bevy::prelude::*;
pub use crate::{GameAssets, Movement, enemy_type::EnemyType};
use crate::{AnimationIndices, AnimationTimer, GameState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Enemy>()
      .add_event::<EnemyDeathEvent>()
      .add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(despawn_enemy_on_death));
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
      name: Name::new("GreenEnemy")
    }
  }
}

// !!! Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy {
  pub health: i32
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
  enemy_type: EnemyType,
  assets: &GameAssets,
  position: Vec3,
  direction: Vec3
) {
  commands.spawn(enemy_type.get_enemy(assets, position, direction));
}

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