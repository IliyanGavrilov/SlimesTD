use crate::*;
use bevy::utils::HashMap;

fn stats_with_green() -> EnemyTypeStats {
  let mut enemy = HashMap::new();
  enemy.insert(EnemyType::Green, EnemyBundle::default());
  EnemyTypeStats { enemy }
}

fn three_point_map() -> Map {
  Map {
    checkpoints: vec![
      Vec3::new(0., 0., 0.),
      Vec3::new(10., 0., 0.),
      Vec3::new(10., 10., 0.),
    ],
    ..Default::default()
  }
}

#[test]
fn direction_at_first_index_points_to_next_checkpoint() {
  let enemy = EnemyType::Green.get_enemy(
    &three_point_map(),
    Path { index: 0, route: 0 },
    &stats_with_green(),
  );
  assert_eq!(enemy.movement.direction, Vec3::new(10., 0., 0.));
}

#[test]
fn direction_at_second_index_points_to_following_checkpoint() {
  let enemy = EnemyType::Green.get_enemy(
    &three_point_map(),
    Path { index: 1, route: 0 },
    &stats_with_green(),
  );
  assert_eq!(enemy.movement.direction, Vec3::new(10., 10., 0.));
}

#[test]
fn get_enemy_keeps_the_given_index() {
  let enemy = EnemyType::Green.get_enemy(
    &three_point_map(),
    Path { index: 1, route: 0 },
    &stats_with_green(),
  );
  assert_eq!(enemy.path.index, 1);
}

#[test]
fn get_enemy_keeps_the_given_route() {
  let enemy = EnemyType::Green.get_enemy(
    &three_point_map(),
    Path { index: 0, route: 0 },
    &stats_with_green(),
  );
  assert_eq!(enemy.path.route, 0);
}

#[test]
fn get_enemy_clones_health_from_stats() {
  let enemy = EnemyType::Green.get_enemy(
    &three_point_map(),
    Path { index: 0, route: 0 },
    &stats_with_green(),
  );
  assert_eq!(enemy.enemy.health, 1);
}

#[test]
#[should_panic]
fn get_enemy_panics_past_the_last_checkpoint() {
  let map = Map {
    checkpoints: vec![Vec3::ZERO, Vec3::ONE],
    ..Default::default()
  };
  let _ = EnemyType::Green.get_enemy(&map, Path { index: 1, route: 0 }, &stats_with_green());
}
