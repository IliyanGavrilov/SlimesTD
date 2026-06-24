use bevy::prelude::*;

use crate::{
  game_not_paused, EnemyDeathEvent, FloatingTextEvent, GameState, Player, WaveClearedEvent,
};

pub struct FarmPlugin;

impl Plugin for FarmPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<FarmTower>().add_system(
      farm_income
        .in_set(OnUpdate(GameState::Gameplay))
        .run_if(game_not_paused),
    );
  }
}

/// Determines how a farm tower generates income.
/// Each variant encapsulates its own state so the income system stays generic.
#[derive(Clone)]
pub enum FarmBehavior {
  /// Earns a fixed amount every interval (seconds).
  Passive { income: u32, timer: Timer },
  /// Earns a fixed amount for every enemy killed anywhere on the map.
  Kill { income_per_kill: u32 },
  /// Earns a fixed amount each time a wave is cleared.
  Wave { income_per_wave: u32 },
  /// Earns a fixed amount for every enemy this tower personally kills.
  SelfKill { income_per_kill: u32 },
}

#[derive(Reflect, Component, Clone)]
#[reflect(Component)]
pub struct FarmTower {
  #[reflect(ignore)]
  pub behavior: FarmBehavior,
}

impl Default for FarmTower {
  fn default() -> Self {
    Self {
      behavior: FarmBehavior::Passive {
        income: 50,
        timer: Timer::from_seconds(15.0, TimerMode::Repeating),
      },
    }
  }
}

impl FarmTower {
  pub fn new(behavior: FarmBehavior) -> Self {
    Self { behavior }
  }

  /// Applies an income upgrade to whichever behavior this farm uses.
  pub fn apply_income_upgrade(&mut self, amount: u32) {
    match &mut self.behavior {
      FarmBehavior::Passive { income, .. } => *income += amount,
      FarmBehavior::Kill { income_per_kill } => *income_per_kill += amount,
      FarmBehavior::Wave { income_per_wave } => *income_per_wave += amount,
      FarmBehavior::SelfKill { income_per_kill } => *income_per_kill += amount,
    }
  }
}

fn farm_income(
  mut farms: Query<(&mut FarmTower, &Transform)>,
  mut player: Query<&mut Player>,
  time: Res<Time>,
  mut death_events: EventReader<EnemyDeathEvent>,
  mut wave_events: EventReader<WaveClearedEvent>,
  mut float_writer: EventWriter<FloatingTextEvent>,
) {
  let kills = death_events.iter().count();
  let waves_cleared = wave_events.iter().count();
  let mut player = player.single_mut();

  for (mut farm, transform) in &mut farms {
    // Income earned by this farm this tick; 0 means no indicator.
    let earned = match &mut farm.behavior {
      FarmBehavior::Passive { income, timer } => {
        timer.tick(time.delta());
        if timer.just_finished() {
          *income as usize
        } else {
          0
        }
      }
      FarmBehavior::Kill { income_per_kill } => kills * *income_per_kill as usize,
      FarmBehavior::Wave { income_per_wave } => waves_cleared * *income_per_wave as usize,
      FarmBehavior::SelfKill { .. } => 0, // awarded per-kill inside bullet_enemy_collision
    };

    if earned > 0 {
      player.money += earned;
      float_writer.send(FloatingTextEvent {
        position: transform.translation,
        text: format!("+${}", earned),
        color: Color::GOLD,
      });
    }
  }
}

#[cfg(test)]
#[path = "farm_tower/farm_tests.rs"]
mod tests;
