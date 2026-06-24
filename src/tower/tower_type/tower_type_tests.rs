use super::*;

#[test]
fn dark_description_mentions_invisible() {
  assert_eq!(
    TowerType::Dark.description(),
    "Knocks back; sees invisible enemies"
  );
}

#[test]
fn nature_description_mentions_poison() {
  assert_eq!(TowerType::Nature.description(), "Poisons enemies over time");
}

#[test]
fn farm_passive_has_a_farm_behavior() {
  assert!(TowerType::FarmPassive.get_farm_tower().is_some());
}

#[test]
fn farm_kill_has_a_farm_behavior() {
  assert!(TowerType::FarmKill.get_farm_tower().is_some());
}

#[test]
fn farm_wave_has_a_farm_behavior() {
  assert!(TowerType::FarmWave.get_farm_tower().is_some());
}

#[test]
fn farm_self_kill_has_a_farm_behavior() {
  assert!(TowerType::FarmSelfKill.get_farm_tower().is_some());
}

#[test]
fn nature_is_not_a_farm() {
  assert!(TowerType::Nature.get_farm_tower().is_none());
}

#[test]
fn dark_is_not_a_farm() {
  assert!(TowerType::Dark.get_farm_tower().is_none());
}

#[test]
fn farm_passive_earns_fifty() {
  let farm = TowerType::FarmPassive.get_farm_tower().unwrap();
  assert!(matches!(
    farm.behavior,
    FarmBehavior::Passive { income: 50, .. }
  ));
}

#[test]
fn farm_kill_earns_five_per_kill() {
  let farm = TowerType::FarmKill.get_farm_tower().unwrap();
  assert!(matches!(
    farm.behavior,
    FarmBehavior::Kill { income_per_kill: 5 }
  ));
}

#[test]
fn farm_wave_earns_seventy_five_per_wave() {
  let farm = TowerType::FarmWave.get_farm_tower().unwrap();
  assert!(matches!(
    farm.behavior,
    FarmBehavior::Wave {
      income_per_wave: 75
    }
  ));
}

#[test]
fn farm_self_kill_earns_three_per_kill() {
  let farm = TowerType::FarmSelfKill.get_farm_tower().unwrap();
  assert!(matches!(
    farm.behavior,
    FarmBehavior::SelfKill { income_per_kill: 3 }
  ));
}

#[test]
fn combat_tower_has_default_sprite_scale() {
  assert_eq!(TowerType::Nature.sprite_scale(), 1.0);
}

#[test]
fn farm_self_kill_has_larger_sprite_scale() {
  assert_eq!(TowerType::FarmSelfKill.sprite_scale(), 1.5);
}

#[test]
fn farm_wave_has_smaller_sprite_scale() {
  assert_eq!(TowerType::FarmWave.sprite_scale(), 0.75);
}

#[test]
fn combat_tower_placement_radius_is_twenty() {
  assert_eq!(TowerType::Nature.placement_radius(), 20.0);
}

#[test]
fn farm_self_kill_placement_radius_scales_up() {
  assert_eq!(TowerType::FarmSelfKill.placement_radius(), 30.0);
}

#[test]
fn farm_wave_placement_radius_scales_down() {
  assert_eq!(TowerType::FarmWave.placement_radius(), 15.0);
}
