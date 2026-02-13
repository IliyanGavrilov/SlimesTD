use std::time::Duration;
use crate::*;

#[test]
fn test_wave_progression() {
    let mut waves = Waves {
        waves: vec![
            Wave { enemies: vec![(EnemyType::Green, Duration::from_secs(1))], current: 0 },
            Wave { enemies: vec![(EnemyType::Red, Duration::from_secs(1))], current: 0 },
        ],
        current: 0,
    };
    waves.current += 1;
    let current_wave = waves.current().unwrap();
    assert_eq!(current_wave.enemies[0].0, EnemyType::Red);
}

#[test]
fn test_waves_current_returns_none_when_exhausted() {
    let waves = Waves {
        waves: vec![Wave::default()],
        current: 1,
    };
    assert!(waves.current().is_none());
}

#[test]
fn test_spawn_index_calculation() {
    let enemies = vec![
        (EnemyType::Green, Duration::from_secs(1)),
        (EnemyType::Red, Duration::from_secs(2)),
        (EnemyType::Blue, Duration::from_secs(3)),
    ];
    let total = enemies.len();
    let remaining_counts = vec![3, 2, 1];
    let expected_indices = vec![0, 1, 2];
    for (remaining, expected) in remaining_counts.iter().zip(expected_indices) {
        let index = total - remaining;
        assert_eq!(index, expected);
    }
}

#[test]
fn test_wave_state_initialization_from_wave() {
    let wave = Wave {
        enemies: vec![
            (EnemyType::Green, Duration::from_secs(5)),
            (EnemyType::Red, Duration::from_secs(2)),
        ],
        current: 0,
    };
    let state = WaveState::from((&wave, wave.enemies.len()));
    assert_eq!(state.remaining, 2);
    assert_eq!(state.enemy_spawn_timer.duration(), Duration::from_secs(5));
}