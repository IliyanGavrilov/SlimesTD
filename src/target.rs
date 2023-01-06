use bevy::utils::FloatOrd;

// Debugging
use bevy::prelude::*;
use crate::Enemy;

#[derive(Reflect, Component, Default)]
pub enum TargetingPriority {
  #[default]
  FIRST,
  LAST,
  CLOSE,
  STRONGEST,
  WEAKEST
}

pub fn first_enemy_direction(
  _enemies: &Query<&GlobalTransform, With<Enemy>>,
  _bullet_spawn_pos: Vec3,
) -> Option<Vec3> {
  return None;
}

pub fn last_enemy_direction(
  _enemies: &Query<&GlobalTransform, With<Enemy>>,
  _bullet_spawn_pos: Vec3,
) -> Option<Vec3> {
  return None;
}

pub fn closest_enemy_direction(
  enemies: &Query<&GlobalTransform, With<Enemy>>,
  bullet_spawn_pos: Vec3,
  
) -> Option<Vec3> {
  let closest_enemy = enemies
    .iter()
    .min_by_key(|enemy_transform| { // Find closest enemy
      FloatOrd(Vec3::distance(enemy_transform.translation(), bullet_spawn_pos))
    });
  
  if let Some(closest_enemy) = closest_enemy {
    return Option::from(closest_enemy.translation() - bullet_spawn_pos); // return direction
  }
  return None;
}

pub fn strongest_enemy_direction(
  _enemies: &Query<&GlobalTransform, With<Enemy>>,
  _bullet_spawn_pos: Vec3,
) -> Option<Vec3> {
  return None;
}

pub fn weakest_enemy_direction(
  _enemies: &Query<&GlobalTransform, With<Enemy>>,
  _bullet_spawn_pos: Vec3,
) -> Option<Vec3> {
  return None;
}