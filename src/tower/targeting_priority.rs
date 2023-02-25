use bevy::prelude::*;
use bevy::utils::FloatOrd;
use strum_macros::EnumIter;
use serde::{Serialize, Deserialize};

use crate::enemy::*;
use crate::movement::*;

#[derive(EnumIter, Reflect, Clone, Debug, Component, Default, PartialEq, Serialize, Deserialize)]
pub enum TargetingPriority {
  #[default]
  FIRST,
  LAST,
  CLOSE,
  STRONG,
  WEAK
}

impl TargetingPriority {
  pub fn next_target(&mut self) {
    *self = match self {
      TargetingPriority::FIRST => TargetingPriority::LAST,
      TargetingPriority::LAST => TargetingPriority::CLOSE,
      TargetingPriority::CLOSE => TargetingPriority::STRONG,
      TargetingPriority::STRONG => TargetingPriority::WEAK,
      TargetingPriority::WEAK => TargetingPriority::FIRST
    }
  }
  
  pub fn prev_target(&mut self) {
    *self = match self {
      TargetingPriority::FIRST => TargetingPriority::WEAK,
      TargetingPriority::LAST => TargetingPriority::FIRST,
      TargetingPriority::CLOSE => TargetingPriority::LAST,
      TargetingPriority::STRONG => TargetingPriority::CLOSE,
      TargetingPriority::WEAK => TargetingPriority::STRONG
    }
  }
}

pub fn first_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement)>,
  bullet_spawn_pos: Vec3,
  tower_range: u32
) -> Option<Vec3> {
  let first_enemy = enemies
    .iter()
    .filter(|(enemy_transform, ..)| {
      Vec3::distance(enemy_transform.translation(),
                     bullet_spawn_pos) <= tower_range as f32
    })
    // Find first enemy that is closest to the base
    .max_by_key(|(.., movement)| {
      FloatOrd(movement.distance_travelled)
    });
  
  if let Some((first_enemy, ..)) = first_enemy {
    return Option::from(first_enemy.translation() - bullet_spawn_pos); // return direction
  }
  return None;
}

pub fn last_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement)>,
  bullet_spawn_pos: Vec3,
  tower_range: u32
) -> Option<Vec3> {
  let last_enemy = enemies
    .iter()
    .filter(|(enemy_transform, ..)| {
      Vec3::distance(enemy_transform.translation(),
                     bullet_spawn_pos) <= tower_range as f32
    })
    // Find first enemy that is closest to the base
    .min_by_key(|(.., movement)| {
      FloatOrd(movement.distance_travelled)
    });
  
  if let Some((last_enemy, ..)) = last_enemy {
    return Option::from(last_enemy.translation() - bullet_spawn_pos); // return direction
  }
  return None;
}

pub fn closest_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement)>,
  bullet_spawn_pos: Vec3,
  tower_range: u32
) -> Option<Vec3> {
  let closest_enemy = enemies
    .iter()
    .filter(|(enemy_transform, ..)| {
      Vec3::distance(enemy_transform.translation(),
                     bullet_spawn_pos) <= tower_range as f32
    })
    .min_by_key(|(enemy_transform, ..)| { // Find closest enemy
      FloatOrd(Vec3::distance(enemy_transform.translation(), bullet_spawn_pos))
    });
  
  if let Some((closest_enemy, ..)) = closest_enemy {
    return Option::from(closest_enemy.translation() - bullet_spawn_pos); // return direction
  }
  return None;
}

pub fn strongest_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement)>,
  bullet_spawn_pos: Vec3,
  tower_range: u32
) -> Option<Vec3> {
  let strongest_enemy = enemies
    .iter()
    .filter(|(enemy_transform, ..)| {
      Vec3::distance(enemy_transform.translation(),
                     bullet_spawn_pos) <= tower_range as f32
    })
    .max_by_key(|(_, enemy, ..)| { // Find strongest enemy
      FloatOrd(enemy.health as f32)
    });
  
  if let Some((strongest_enemy, ..)) = strongest_enemy {
    return Option::from(strongest_enemy.translation() - bullet_spawn_pos); // return direction
  }
  return None;
}

pub fn weakest_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement)>,
  bullet_spawn_pos: Vec3,
  tower_range: u32
) -> Option<Vec3> {
  let weakest_enemy = enemies
    .iter()
    .filter(|(enemy_transform, ..)| {
      Vec3::distance(enemy_transform.translation(),
                     bullet_spawn_pos) <= tower_range as f32
    })
    .min_by_key(|(_, enemy, ..)| { // Find weakest enemy
      FloatOrd(enemy.health as f32)
    });
  
  if let Some((weakest_enemy, ..)) = weakest_enemy {
    return Option::from(weakest_enemy.translation() - bullet_spawn_pos); // return direction
  }
  return None;
}