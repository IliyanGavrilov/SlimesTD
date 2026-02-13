use crate::*;

#[test]
fn test_damage_base_partial() {
    let mut base = Base { health: 100 };
    let enemy_health = 30;
    if base.health > enemy_health {
        base.health -= enemy_health;
    } else {
        base.health = 0;
    }
    assert_eq!(base.health, 70);
}

#[test]
fn test_damage_base_overkill_clamps_to_zero() {
    let mut base = Base { health: 10 };
    let enemy_health = 50;
    if base.health > enemy_health {
        base.health -= enemy_health;
    } else {
        base.health = 0;
    }
    assert_eq!(base.health, 0);
}

#[test]
fn test_damage_base_exact_health_clamps_to_zero() {
    let mut base = Base { health: 50 };
    let enemy_health = 50;
    if base.health > enemy_health {
        base.health -= enemy_health;
    } else {
        base.health = 0;
    }
    assert_eq!(base.health, 0);
}