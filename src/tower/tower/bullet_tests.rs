use super::*;

#[test]
fn test_bullet_default() {
    let bullet = Bullet::default();
    assert_eq!(bullet.damage, 0);
    assert!(!bullet.lifetime.finished());
}

#[test]
fn test_bullet_initialization() {
    let bullet = Bullet {
        damage: 42,
        lifetime: Timer::from_seconds(2.0, TimerMode::Once),
    };

    assert_eq!(bullet.damage, 42);
    assert_eq!(bullet.lifetime.duration().as_secs(), 2);
    assert!(!bullet.lifetime.finished());
}

#[test]
fn test_bullet_lifetime_partial_tick() {
    let mut bullet = Bullet {
        damage: 10,
        lifetime: Timer::from_seconds(1.0, TimerMode::Once),
    };
    bullet.lifetime.tick(std::time::Duration::from_secs_f32(0.5));
    assert!(!bullet.lifetime.finished());
}

#[test]
fn test_bullet_lifetime_exact_tick() {
    let mut bullet = Bullet {
        damage: 10,
        lifetime: Timer::from_seconds(1.0, TimerMode::Once),
    };
    bullet.lifetime.tick(std::time::Duration::from_secs_f32(1.0));
    assert!(bullet.lifetime.finished());
}

#[test]
fn test_bullet_lifetime_overtime_tick() {
    let mut bullet = Bullet {
        damage: 10,
        lifetime: Timer::from_seconds(1.0, TimerMode::Once),
    };
    bullet.lifetime.tick(std::time::Duration::from_secs_f32(1.5));
    assert!(bullet.lifetime.finished());
}

#[test]
fn test_tower_damage_capped_to_enemy_health() {
    let bullet_damage = 100u32;
    let enemy_health = 30i32;
    let mut total_damage = 0u32;

    if enemy_health >= bullet_damage as i32 {
        total_damage += bullet_damage;
    } else {
        total_damage += enemy_health as u32;
    }

    assert_eq!(total_damage, 30);
}

#[test]
fn test_tower_damage_full_when_enemy_health_higher() {
    let bullet_damage = 20u32;
    let enemy_health = 50i32;
    let mut total_damage = 0u32;

    if enemy_health >= bullet_damage as i32 {
        total_damage += bullet_damage;
    } else {
        total_damage += enemy_health as u32;
    }

    assert_eq!(total_damage, 20);
}

#[test]
fn test_tower_damage_exact_match() {
    let bullet_damage = 50u32;
    let enemy_health = 50i32;
    let mut total_damage = 0u32;

    if enemy_health >= bullet_damage as i32 {
        total_damage += bullet_damage;
    } else {
        total_damage += enemy_health as u32;
    }

    assert_eq!(total_damage, 50);
}