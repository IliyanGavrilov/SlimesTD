use bevy::prelude::*;
use bevy::utils::FloatOrd;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
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
  fn as_index(&self) -> usize {
    Self::iter().position(|v| v == *self).unwrap()
  }

  fn from_index(idx: usize) -> Self {
    let count = Self::iter().count();
    Self::iter().nth(idx % count).unwrap()
  }

  pub fn next_target(&mut self) {
    let count = Self::iter().count();
    *self = Self::from_index((self.as_index() + 1) % count);
  }

  pub fn prev_target(&mut self) {
    let count = Self::iter().count();
    *self = Self::from_index((self.as_index() + count - 1) % count);
  }
}

/// Whether a tower may target an enemy. Invisible enemies are hidden from every
/// tower except those that can see them (the Dark tower).
pub fn is_targetable(is_invisible: bool, can_see_invisible: bool) -> bool {
  can_see_invisible || !is_invisible
}

pub fn get_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement, Option<&Invisible>)>,
  bullet_spawn_pos: Vec3,
  tower_range: u32,
  tower_targeting_priority: &TargetingPriority,
  can_see_invisible: bool,
) -> Option<Vec3> {
  let enemy_filtered_query = enemies
    .iter()
    // Within range and visible to this tower.
    .filter(|(enemy_transform, _, _, invisible)| {
      is_targetable(invisible.is_some(), can_see_invisible)
        && Vec3::distance(enemy_transform.translation(), bullet_spawn_pos) <= tower_range as f32
    });

  let enemy = match tower_targeting_priority {
    TargetingPriority::FIRST => enemy_filtered_query
      // Furthest along the path (closest to the base).
      .max_by_key(|(_, _, movement, _)| FloatOrd(movement.distance_travelled)),
    TargetingPriority::LAST => enemy_filtered_query
      // Least far along the path (nearest the spawn).
      .min_by_key(|(_, _, movement, _)| FloatOrd(movement.distance_travelled)),
    TargetingPriority::CLOSE => enemy_filtered_query
      // Closest to the tower.
      .min_by_key(|(enemy_transform, ..)| {
        FloatOrd(Vec3::distance(
          enemy_transform.translation(),
          bullet_spawn_pos,
        ))
      }),
    TargetingPriority::FAR => enemy_filtered_query
      // Farthest from the tower.
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
      .choose(&mut rand::rng())
  };

  if let Some((enemy, ..)) = enemy {
    return Option::from(enemy.translation() - bullet_spawn_pos);
  }
  None
}

#[cfg(test)]
#[path = "tower/targeting_tests.rs"]
mod tests;