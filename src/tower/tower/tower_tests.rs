use super::*;

#[test]
fn test_tower_new_initialization() {
    let offset = Vec3::new(10.0, 5.0, 0.0);
    let tower = Tower::new(offset, 25, 1.5, 100, 200);

    assert_eq!(tower.bullet_spawn_offset, offset);
    assert_eq!(tower.damage, 25);
    assert_eq!(tower.attack_speed, 1.5);
    assert_eq!(tower.range, 100);
    assert_eq!(tower.price, 200);
    assert_eq!(tower.total_spent, 200);
    assert_eq!(tower.sell_price, 66);
    assert!(!tower.first_enemy_appeared);
    assert_eq!(tower.total_damage, 0);
}

#[test]
fn test_tower_sell_price_calculation() {
    let tower = Tower::new(Vec3::ZERO, 10, 1.0, 50, 300);
    assert_eq!(tower.sell_price, 100);
}

#[test]
fn test_tower_sell_price_rounds_down() {
    let tower = Tower::new(Vec3::ZERO, 10, 1.0, 50, 100);
    assert_eq!(tower.sell_price, 33);
}

#[test]
fn test_tower_shooting_timer_duration() {
    let tower = Tower::new(Vec3::ZERO, 10, 2.0, 50, 100);
    assert_eq!(tower.shooting_timer.duration(), Duration::from_millis(2000));
}

#[test]
fn test_tower_shooting_timer_repeating_mode() {
    let tower = Tower::new(Vec3::ZERO, 10, 1.0, 50, 100);
    assert_eq!(tower.shooting_timer.mode(), TimerMode::Repeating);
}

#[test]
fn test_tower_default_values() {
    let tower = Tower::default();
    assert_eq!(tower.damage, 0);
    assert_eq!(tower.range, 0);
    assert_eq!(tower.price, 0);
    assert_eq!(tower.total_damage, 0);
    assert_eq!(tower.total_spent, 0);
}

#[test]
fn test_tower_shooting_timer_tick_behavior() {
    let mut tower = Tower::new(Vec3::ZERO, 10, 1.0, 50, 100);

    tower.shooting_timer.tick(Duration::from_millis(500));
    assert!(!tower.shooting_timer.finished());

    tower.shooting_timer.tick(Duration::from_millis(600));
    assert!(tower.shooting_timer.finished());
}

#[test]
fn test_tower_shooting_timer_just_finished() {
    let mut tower = Tower::new(Vec3::ZERO, 10, 1.0, 50, 100);

    tower.shooting_timer.tick(Duration::from_millis(999));
    assert!(!tower.shooting_timer.just_finished());

    tower.shooting_timer.tick(Duration::from_millis(1));
    assert!(tower.shooting_timer.just_finished());
}

#[test]
fn test_tower_attack_speed_upgrade_calculation() {
    let mut tower = Tower::new(Vec3::ZERO, 10, 1.0, 100, 300);

    let upgrade_value = 10.0;
    tower.attack_speed -= upgrade_value * 0.01 * tower.attack_speed;

    assert_eq!(tower.attack_speed, 0.9);
}

#[test]
fn test_tower_range_detection() {
    let tower_pos = Vec3::new(0.0, 0.0, 0.0);
    let enemy_in_range = Vec3::new(30.0, 0.0, 0.0);
    let enemy_out_of_range = Vec3::new(200.0, 0.0, 0.0);
    let range = 100.0;

    assert!(tower_pos.distance(enemy_in_range) <= range);
    assert!(tower_pos.distance(enemy_out_of_range) > range);
}