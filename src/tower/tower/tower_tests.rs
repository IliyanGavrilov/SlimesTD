use super::*;

#[test]
fn new_sets_bullet_spawn_offset() {
  let offset = Vec3::new(10.0, 5.0, 0.0);
  let tower = Tower::new(offset, 25, 1.5, 100, 200);
  assert_eq!(tower.bullet_spawn_offset, offset);
}

#[test]
fn new_sets_damage() {
  let tower = Tower::new(Vec3::ZERO, 25, 1.5, 100, 200);
  assert_eq!(tower.damage, 25);
}

#[test]
fn new_sets_attack_speed() {
  let tower = Tower::new(Vec3::ZERO, 25, 1.5, 100, 200);
  assert_eq!(tower.attack_speed, 1.5);
}

#[test]
fn new_sets_range() {
  let tower = Tower::new(Vec3::ZERO, 25, 1.5, 100, 200);
  assert_eq!(tower.range, 100);
}

#[test]
fn new_sets_price() {
  let tower = Tower::new(Vec3::ZERO, 25, 1.5, 100, 200);
  assert_eq!(tower.price, 200);
}

#[test]
fn new_sets_total_spent_to_price() {
  let tower = Tower::new(Vec3::ZERO, 25, 1.5, 100, 200);
  assert_eq!(tower.total_spent, 200);
}

#[test]
fn new_sets_sell_price_to_a_third_of_price() {
  let tower = Tower::new(Vec3::ZERO, 25, 1.5, 100, 300);
  assert_eq!(tower.sell_price, 100);
}

#[test]
fn new_starts_with_first_enemy_not_appeared() {
  let tower = Tower::new(Vec3::ZERO, 25, 1.5, 100, 200);
  assert!(!tower.first_enemy_appeared);
}

#[test]
fn new_starts_with_zero_total_damage() {
  let tower = Tower::new(Vec3::ZERO, 25, 1.5, 100, 200);
  assert_eq!(tower.total_damage, 0);
}

#[test]
fn new_starts_with_zero_pierce() {
  let tower = Tower::new(Vec3::ZERO, 25, 1.5, 100, 200);
  assert_eq!(tower.pierce, 0);
}

#[test]
fn new_cannot_see_invisible_by_default() {
  let tower = Tower::new(Vec3::ZERO, 25, 1.5, 100, 200);
  assert!(!tower.can_see_invisible);
}

#[test]
fn new_has_no_effect_by_default() {
  let tower = Tower::new(Vec3::ZERO, 25, 1.5, 100, 200);
  assert!(tower.effect.is_none());
}

#[test]
fn sell_price_rounds_down() {
  let tower = Tower::new(Vec3::ZERO, 10, 1.0, 50, 100);
  assert_eq!(tower.sell_price, 33);
}

#[test]
fn sell_price_of_cheap_tower_is_zero() {
  let tower = Tower::new(Vec3::ZERO, 10, 1.0, 50, 2);
  assert_eq!(tower.sell_price, 0);
}

#[test]
fn sell_price_of_free_tower_is_zero() {
  let tower = Tower::new(Vec3::ZERO, 10, 1.0, 50, 0);
  assert_eq!(tower.sell_price, 0);
}

#[test]
fn shooting_timer_duration_matches_attack_speed_of_one() {
  let tower = Tower::new(Vec3::ZERO, 10, 1.0, 50, 100);
  assert_eq!(tower.shooting_timer.duration(), Duration::from_millis(1000));
}

#[test]
fn shooting_timer_duration_matches_attack_speed_of_two() {
  let tower = Tower::new(Vec3::ZERO, 10, 2.0, 50, 100);
  assert_eq!(tower.shooting_timer.duration(), Duration::from_millis(2000));
}

#[test]
fn shooting_timer_duration_matches_fractional_attack_speed() {
  let tower = Tower::new(Vec3::ZERO, 10, 0.5, 50, 100);
  assert_eq!(tower.shooting_timer.duration(), Duration::from_millis(500));
}

#[test]
fn shooting_timer_is_repeating() {
  let tower = Tower::new(Vec3::ZERO, 10, 1.0, 50, 100);
  assert_eq!(tower.shooting_timer.mode(), TimerMode::Repeating);
}

#[test]
fn shooting_timer_not_finished_before_duration() {
  let mut tower = Tower::new(Vec3::ZERO, 10, 1.0, 50, 100);
  tower.shooting_timer.tick(Duration::from_millis(999));
  assert!(!tower.shooting_timer.finished());
}

#[test]
fn shooting_timer_finished_at_duration() {
  let mut tower = Tower::new(Vec3::ZERO, 10, 1.0, 50, 100);
  tower.shooting_timer.tick(Duration::from_millis(1000));
  assert!(tower.shooting_timer.finished());
}

#[test]
fn default_tower_has_zero_damage() {
  assert_eq!(Tower::default().damage, 0);
}

#[test]
fn default_tower_has_zero_range() {
  assert_eq!(Tower::default().range, 0);
}

#[test]
fn default_tower_has_zero_price() {
  assert_eq!(Tower::default().price, 0);
}

#[test]
fn default_tower_has_zero_sell_price() {
  assert_eq!(Tower::default().sell_price, 0);
}

#[test]
fn default_tower_cannot_see_invisible() {
  assert!(!Tower::default().can_see_invisible);
}
