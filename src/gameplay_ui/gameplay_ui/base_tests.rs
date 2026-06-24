use crate::*;

#[test]
fn partial_damage_reduces_health() {
  let mut base = Base { health: 100 };
  base.take_damage(30);
  assert_eq!(base.health, 70);
}

#[test]
fn overkill_clamps_health_to_zero() {
  let mut base = Base { health: 10 };
  base.take_damage(50);
  assert_eq!(base.health, 0);
}

#[test]
fn exact_lethal_damage_clamps_to_zero() {
  let mut base = Base { health: 50 };
  base.take_damage(50);
  assert_eq!(base.health, 0);
}

#[test]
fn zero_damage_leaves_health_unchanged() {
  let mut base = Base { health: 100 };
  base.take_damage(0);
  assert_eq!(base.health, 100);
}

#[test]
fn one_health_against_one_damage_dies() {
  let mut base = Base { health: 1 };
  base.take_damage(1);
  assert_eq!(base.health, 0);
}

#[test]
fn one_health_against_no_damage_survives() {
  let mut base = Base { health: 1 };
  base.take_damage(0);
  assert_eq!(base.health, 1);
}
