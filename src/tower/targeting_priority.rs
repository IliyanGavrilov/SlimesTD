use bevy::prelude::*;
use bevy::utils::FloatOrd;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::enemy::*;
use crate::movement::*;

#[derive(
  EnumIter, Reflect, Clone, Debug, Component, Default, PartialEq, Serialize, Deserialize,
)]
pub enum TargetingPriority {
  #[default]
  FIRST,
  LAST,
  CLOSE,
  FAR,
  STRONG,
  WEAK,
  RANDOM,
}

impl TargetingPriority {
  pub fn next_target(&mut self) {
    *self = match self {
      TargetingPriority::FIRST => TargetingPriority::LAST,
      TargetingPriority::LAST => TargetingPriority::CLOSE,
      TargetingPriority::CLOSE => TargetingPriority::FAR,
      TargetingPriority::FAR => TargetingPriority::STRONG,
      TargetingPriority::STRONG => TargetingPriority::WEAK,
      TargetingPriority::WEAK => TargetingPriority::RANDOM,
      TargetingPriority::RANDOM => TargetingPriority::FIRST,
    }
  }

  pub fn prev_target(&mut self) {
    *self = match self {
      TargetingPriority::FIRST => TargetingPriority::RANDOM,
      TargetingPriority::LAST => TargetingPriority::FIRST,
      TargetingPriority::CLOSE => TargetingPriority::LAST,
      TargetingPriority::FAR => TargetingPriority::CLOSE,
      TargetingPriority::STRONG => TargetingPriority::FAR,
      TargetingPriority::WEAK => TargetingPriority::STRONG,
      TargetingPriority::RANDOM => TargetingPriority::WEAK,
    }
  }
}

pub fn get_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement)>,
  bullet_spawn_pos: Vec3,
  tower_range: u32,
  tower_targeting_priority: &TargetingPriority,
) -> Option<Vec3> {
  let enemy_filtered_query = enemies
    .iter()
    // Filter the enemies that are in the tower's range
    .filter(|(enemy_transform, ..)| {
      Vec3::distance(enemy_transform.translation(), bullet_spawn_pos) <= tower_range as f32
    });

  let enemy = match tower_targeting_priority {
    TargetingPriority::FIRST => enemy_filtered_query
      // Find first enemy that is closest to the base
      .max_by_key(|(.., movement)| FloatOrd(movement.distance_travelled)),
    TargetingPriority::LAST => enemy_filtered_query
      // Find first enemy that is closest to the base
      .min_by_key(|(.., movement)| FloatOrd(movement.distance_travelled)),
    TargetingPriority::CLOSE => enemy_filtered_query
      // Find enemy that is closest to the tower
      .min_by_key(|(enemy_transform, ..)| {
        FloatOrd(Vec3::distance(
          enemy_transform.translation(),
          bullet_spawn_pos,
        ))
      }),
    TargetingPriority::FAR => enemy_filtered_query
      // Find enemy that is the farthest away from the tower
      .max_by_key(|(enemy_transform, ..)| {
        FloatOrd(Vec3::distance(
          enemy_transform.translation(),
          bullet_spawn_pos,
        ))
      }),
    TargetingPriority::STRONG => enemy_filtered_query
      // Find the strongest enemy
      .max_by_key(|(_, enemy, ..)| FloatOrd(enemy.health as f32)),
    TargetingPriority::WEAK => enemy_filtered_query
      // Find the weakest enemy
      .min_by_key(|(_, enemy, ..)| FloatOrd(enemy.health as f32)),
    TargetingPriority::RANDOM => enemy_filtered_query
      // Choose a random enemy
      .choose(&mut rand::thread_rng()),
  };

  if let Some((enemy, ..)) = enemy {
    // return direction
    return Option::from(enemy.translation() - bullet_spawn_pos);
  }
  None
}
