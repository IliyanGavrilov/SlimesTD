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
  _enemies: &Query<(&GlobalTransform, &Enemy)>,
  _bullet_spawn_pos: Vec3,
  _tower_range: i32
) -> Option<Vec3> {
  return None;
}

pub fn last_enemy_direction(
  _enemies: &Query<(&GlobalTransform, &Enemy)>,
  _bullet_spawn_pos: Vec3,
  _tower_range: i32
) -> Option<Vec3> {
  return None;
}

pub fn closest_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy)>,
  bullet_spawn_pos: Vec3,
  tower_range: i32
) -> Option<Vec3> {
  let closest_enemy = enemies
    .iter()
    .filter(|(enemy_transform, _)| {
      Vec3::distance(enemy_transform.translation(),
                     bullet_spawn_pos) <= tower_range as f32
    })
    .min_by_key(|(enemy_transform, _)| { // Find closest enemy
      FloatOrd(Vec3::distance(enemy_transform.translation(), bullet_spawn_pos))
    });
  
  if let Some((closest_enemy, _)) = closest_enemy {
    return Option::from(closest_enemy.translation() - bullet_spawn_pos); // return direction
  }
  return None;
}

pub fn strongest_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy)>,
  bullet_spawn_pos: Vec3,
  tower_range: i32
) -> Option<Vec3> {
  let strongest_enemy = enemies
    .iter()
    .filter(|(enemy_transform, _)| {
      Vec3::distance(enemy_transform.translation(),
                     bullet_spawn_pos) <= tower_range as f32
    })
    .min_by_key(|(_, enemy)| { // Find strongest enemy
      Reverse(FloatOrd(enemy.health as f32))
    });
  
  if let Some((strongest_enemy, _)) = strongest_enemy {
    return Option::from(strongest_enemy.translation() - bullet_spawn_pos); // return direction
  }
  return None;
}

pub fn weakest_enemy_direction(
  enemies: &Query<(&GlobalTransform, &Enemy)>,
  bullet_spawn_pos: Vec3,
  tower_range: i32
) -> Option<Vec3> {
  let weakest_enemy = enemies
    .iter()
    .filter(|(enemy_transform, _)| {
      Vec3::distance(enemy_transform.translation(),
                     bullet_spawn_pos) <= tower_range as f32
    })
    .min_by_key(|(_, enemy)| { // Find weakest enemy
      FloatOrd(enemy.health as f32)
    });
  
  if let Some((weakest_enemy, _)) = weakest_enemy {
    return Option::from(weakest_enemy.translation() - bullet_spawn_pos); // return direction
  }
  return None;
}