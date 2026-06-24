use super::*;

#[test]
fn default_farm_is_passive_earning_fifty() {
  assert!(matches!(
    FarmTower::default().behavior,
    FarmBehavior::Passive { income: 50, .. }
  ));
}

#[test]
fn new_stores_the_given_behavior() {
  let farm = FarmTower::new(FarmBehavior::Kill { income_per_kill: 5 });
  assert!(matches!(
    farm.behavior,
    FarmBehavior::Kill { income_per_kill: 5 }
  ));
}

#[test]
fn income_upgrade_increases_passive_income() {
  let mut farm = FarmTower::new(FarmBehavior::Passive {
    income: 50,
    timer: Timer::from_seconds(15.0, TimerMode::Repeating),
  });
  farm.apply_income_upgrade(10);
  assert!(matches!(
    farm.behavior,
    FarmBehavior::Passive { income: 60, .. }
  ));
}

#[test]
fn income_upgrade_increases_kill_income() {
  let mut farm = FarmTower::new(FarmBehavior::Kill { income_per_kill: 5 });
  farm.apply_income_upgrade(10);
  assert!(matches!(
    farm.behavior,
    FarmBehavior::Kill {
      income_per_kill: 15
    }
  ));
}

#[test]
fn income_upgrade_increases_wave_income() {
  let mut farm = FarmTower::new(FarmBehavior::Wave {
    income_per_wave: 75,
  });
  farm.apply_income_upgrade(25);
  assert!(matches!(
    farm.behavior,
    FarmBehavior::Wave {
      income_per_wave: 100
    }
  ));
}

#[test]
fn income_upgrade_increases_self_kill_income() {
  let mut farm = FarmTower::new(FarmBehavior::SelfKill { income_per_kill: 3 });
  farm.apply_income_upgrade(7);
  assert!(matches!(
    farm.behavior,
    FarmBehavior::SelfKill {
      income_per_kill: 10
    }
  ));
}
