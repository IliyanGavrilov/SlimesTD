use std::cmp::Reverse;
use bevy::utils::FloatOrd;
use bevy_inspector_egui::Inspectable;

// Debugging
use bevy::prelude::*;
pub use crate::enemy::*;

#[derive(Inspectable, Reflect, Component, Default)]
pub enum TargetingPriority {
  #[default]
  FIRST,
  LAST,
  CLOSE,
  STRONG,
  WEAK
}

pub fn first_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement, &TimeAlive)>,
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
    .min_by_key(|(.., movement, enemy)| {
      Reverse(FloatOrd(movement.speed * enemy.time_alive.elapsed_secs())) // S = v * t
    });
  
  if let Some((first_enemy, ..)) = first_enemy {
    return Option::from(first_enemy.translation() - bullet_spawn_pos); // return direction
  }
  return None;
}

pub fn last_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement, &TimeAlive)>,
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
    .min_by_key(|(.., movement, enemy)| {
      FloatOrd(movement.speed * enemy.time_alive.elapsed_secs()) // S = v * t
    });
  
  if let Some((last_enemy, ..)) = last_enemy {
    return Option::from(last_enemy.translation() - bullet_spawn_pos); // return direction
  }
  return None;
}

pub fn closest_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement, &TimeAlive)>,
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
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement, &TimeAlive)>,
  bullet_spawn_pos: Vec3,
  tower_range: u32
) -> Option<Vec3> {
  let strongest_enemy = enemies
    .iter()
    .filter(|(enemy_transform, ..)| {
      Vec3::distance(enemy_transform.translation(),
                     bullet_spawn_pos) <= tower_range as f32
    })
    .min_by_key(|(_, enemy, ..)| { // Find strongest enemy
      Reverse(FloatOrd(enemy.health as f32))
    });
  
  if let Some((strongest_enemy, ..)) = strongest_enemy {
    return Option::from(strongest_enemy.translation() - bullet_spawn_pos); // return direction
  }
  return None;
}

pub fn weakest_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement, &TimeAlive)>,
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