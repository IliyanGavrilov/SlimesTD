use super::*;
use std::time::Duration;

#[test]
fn new_sets_positive_health() {
  assert_eq!(Enemy::new(5).health, 5);
}

#[test]
fn new_sets_zero_health() {
  assert_eq!(Enemy::new(0).health, 0);
}

#[test]
fn new_allows_negative_health() {
  assert_eq!(Enemy::new(-3).health, -3);
}

#[test]
fn invisible_alpha_is_partially_transparent() {
  assert_eq!(INVISIBLE_ALPHA, 0.3);
}

#[test]
fn invisible_trait_equals_itself() {
  assert_eq!(EnemyTrait::Invisible, EnemyTrait::Invisible);
}

#[test]
fn poison_stores_damage_per_second() {
  assert_eq!(Poisoned::new(4, 3.0).dps, 4);
}

#[test]
fn poison_duration_matches_argument() {
  assert_eq!(
    Poisoned::new(4, 3.0).duration.duration(),
    Duration::from_secs_f32(3.0)
  );
}

#[test]
fn poison_ticks_every_second() {
  assert_eq!(
    Poisoned::new(4, 3.0).tick.duration(),
    Duration::from_secs_f32(1.0)
  );
}

#[test]
fn poison_tick_repeats() {
  assert_eq!(Poisoned::new(4, 3.0).tick.mode(), TimerMode::Repeating);
}

#[test]
fn knockback_stores_strength() {
  assert_eq!(KnockedBack::new(350.0).strength, 350.0);
}

#[test]
fn knockback_lasts_a_fraction_of_a_second() {
  assert_eq!(
    KnockedBack::new(350.0).timer.duration(),
    Duration::from_secs_f32(0.15)
  );
}

#[test]
fn knockback_runs_once() {
  assert_eq!(KnockedBack::new(350.0).timer.mode(), TimerMode::Once);
}

#[test]
fn default_enemy_bundle_starts_at_first_checkpoint() {
  assert_eq!(EnemyBundle::default().path.index, 0);
}

#[test]
fn default_enemy_bundle_uses_main_route() {
  assert_eq!(EnemyBundle::default().path.route, 0);
}

#[test]
fn default_enemy_bundle_has_one_health() {
  assert_eq!(EnemyBundle::default().enemy.health, 1);
}

#[test]
fn default_enemy_bundle_has_default_speed() {
  assert_eq!(EnemyBundle::default().movement.speed, 50.0);
}
