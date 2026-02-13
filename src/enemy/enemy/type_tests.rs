use bevy::utils::HashMap;
use crate::*;

#[test]
fn test_enemy_direction_calculation() {
    let map = Map {
        checkpoints: vec![
            Vec3::new(0., 0., 0.),
            Vec3::new(10., 0., 0.),
            Vec3::new(10., 10., 0.)
        ],
        ..Default::default()
    };

    let mut stats_map = bevy::utils::HashMap::new();
    stats_map.insert(EnemyType::Green, EnemyBundle::default());
    let stats = EnemyTypeStats { enemy: stats_map };

    let enemy_at_0 = EnemyType::Green.get_enemy(&map, Path { index: 0 }, &stats);
    assert_eq!(enemy_at_0.movement.direction, Vec3::new(10., 0., 0.));

    let enemy_at_1 = EnemyType::Green.get_enemy(&map, Path { index: 1 }, &stats);
    assert_eq!(enemy_at_1.movement.direction, Vec3::new(10., 10., 0.));
}

#[test]
#[should_panic]
fn test_get_enemy_out_of_bounds_panic() {
    let mut map = Map::default();
    map.checkpoints = vec![Vec3::ZERO, Vec3::ONE];

    let mut enemy_map = HashMap::new();
    enemy_map.insert(EnemyType::Green, EnemyBundle::default());
    let stats = EnemyTypeStats { enemy: enemy_map };

    let path = Path { index: 1 };
    let _ = EnemyType::Green.get_enemy(&map, path, &stats);
}
