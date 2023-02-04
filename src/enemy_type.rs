use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use strum_macros::{Display};
use crate::{AnimationIndices, AnimationTimer, Enemy, Movement};

#[derive(Inspectable, Component, Display, Clone, Copy, Debug, PartialEq)]
pub enum EnemyType {
  Green,
  Yellow,
  Pink,
  White,
  Blue,
  Orange,
  Purple,
  Red
}

impl EnemyType {
  pub fn get_enemy(&self, direction: Vec3) -> (Enemy, Movement, AnimationIndices, AnimationTimer) {
    match self {
      EnemyType::Green => (
        Enemy::new(1),
        Movement {
          direction,
          speed: 15.
        },
        AnimationIndices {first: 0, last: 9},
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
      ),
      EnemyType::Yellow => (
        Enemy::new(2),
        Movement {
          direction,
          speed: 15.
        },
        AnimationIndices {first: 10, last: 19},
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
      ),
      EnemyType::Pink => (
        Enemy::new(3),
        Movement {
          direction,
          speed: 15.
        },
        AnimationIndices {first: 20, last: 29},
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
      ),
      EnemyType::White => (
        Enemy::new(4),
        Movement {
          direction,
          speed: 15.
        },
        AnimationIndices {first: 30, last: 39},
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
      ),
      EnemyType::Blue => (
        Enemy::new(5),
        Movement {
          direction,
          speed: 15.
        },
        AnimationIndices {first: 40, last: 49},
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
      ),
      EnemyType::Orange => (
        Enemy::new(6),
        Movement {
          direction,
          speed: 15.
        },
        AnimationIndices {first: 50, last: 59},
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
      ),
      EnemyType::Purple => (
        Enemy::new(7),
        Movement {
          direction,
          speed: 15.
        },
        AnimationIndices {first: 60, last: 69},
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
      ),
      EnemyType::Red => (
        Enemy::new(8),
        Movement {
          direction,
          speed: 15.
        },
        AnimationIndices {first: 70, last: 79},
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
      )
    }
  }
}