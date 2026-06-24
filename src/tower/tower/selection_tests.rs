use super::*;

#[test]
fn default_upgrades_have_three_paths() {
  assert_eq!(TowerUpgrades::default().upgrades.len(), 3);
}

#[test]
fn default_upgrade_path_0_is_zero() {
  assert_eq!(TowerUpgrades::default().upgrades[0], 0);
}

#[test]
fn default_upgrade_path_1_is_zero() {
  assert_eq!(TowerUpgrades::default().upgrades[1], 0);
}

#[test]
fn default_upgrade_path_2_is_zero() {
  assert_eq!(TowerUpgrades::default().upgrades[2], 0);
}

#[test]
fn upgrading_one_path_sets_that_path() {
  let mut upgrades = TowerUpgrades::default();
  upgrades.upgrades[0] = 2;
  assert_eq!(upgrades.upgrades[0], 2);
}

#[test]
fn upgrading_one_path_leaves_others_untouched() {
  let mut upgrades = TowerUpgrades::default();
  upgrades.upgrades[0] = 2;
  assert_eq!(upgrades.upgrades[2], 0);
}
