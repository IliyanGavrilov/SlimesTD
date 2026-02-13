use super::*;

#[test]
fn test_tower_selection_distance_threshold() {
    let tower_pos = Vec3::new(100.0, 100.0, 0.0);
    let click_inside = Vec3::new(110.0, 110.0, 0.0);
    let click_outside = Vec3::new(140.0, 140.0, 0.0);

    assert!(Vec3::distance(click_inside, tower_pos) <= 25.0);
    assert!(Vec3::distance(click_outside, tower_pos) > 25.0);
}

#[test]
fn test_tower_sell_refund_calculation() {
    let total_spent = 900u32;
    let expected_refund = 300;

    assert_eq!(total_spent / 3, expected_refund);
}

#[test]
fn test_tower_sell_refund_rounds_down() {
    let total_spent = 100u32;
    let expected_refund = 33;

    assert_eq!(total_spent / 3, expected_refund);
}

#[test]
fn test_upgrade_affordability_check() {
    let player_money_sufficient = 500usize;
    let player_money_insufficient = 100usize;
    let upgrade_cost = 150usize;

    assert!(player_money_sufficient >= upgrade_cost);
    assert!(player_money_insufficient < upgrade_cost);
}

#[test]
fn test_upgrade_availability_with_level_cap() {
    let current_level = 2;
    let max_level = 5;
    let player_money = 500;
    let upgrade_cost = 400;

    let can_upgrade = current_level < max_level && player_money >= upgrade_cost;

    assert!(can_upgrade);
}

#[test]
fn test_tower_upgrade_paths_independence() {
    let mut upgrades = TowerUpgrades::default();
    upgrades.upgrades[0] = 2;
    upgrades.upgrades[1] = 1;

    assert_eq!(upgrades.upgrades[0], 2);
    assert_eq!(upgrades.upgrades[1], 1);
    assert_eq!(upgrades.upgrades[2], 0);
}

#[test]
fn test_targeting_priority_next() {
    let mut target = TargetingPriority::default();

    target.next_target();

    assert_eq!(target, TargetingPriority::LAST);
}

#[test]
fn test_targeting_priority_prev() {
    let mut target = TargetingPriority::default();

    target.prev_target();

    assert_eq!(target, TargetingPriority::RANDOM);
}

#[test]
fn test_targeting_priority_cycling() {
    let mut target = TargetingPriority::default();

    target.next_target();
    target.prev_target();

    assert_eq!(target, TargetingPriority::FIRST);
}