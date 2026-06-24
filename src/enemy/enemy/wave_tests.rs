use super::*;
use std::time::Duration;

#[test]
fn current_returns_the_active_wave() {
  let waves = Waves {
    waves: vec![
      Wave {
        enemies: vec![(EnemyType::Green, Duration::from_secs(1), vec![])],
        current: 0,
      },
      Wave {
        enemies: vec![(EnemyType::Red, Duration::from_secs(1), vec![])],
        current: 0,
      },
    ],
    current: 1,
  };
  assert_eq!(waves.current().unwrap().enemies[0].0, EnemyType::Red);
}

#[test]
fn current_is_none_when_exhausted() {
  let waves = Waves {
    waves: vec![Wave::default()],
    current: 1,
  };
  assert!(waves.current().is_none());
}

#[test]
fn wave_state_remaining_matches_enemy_count() {
  let wave = Wave {
    enemies: vec![
      (EnemyType::Green, Duration::from_secs(5), vec![]),
      (EnemyType::Red, Duration::from_secs(2), vec![]),
    ],
    current: 0,
  };
  let state = WaveState::from((&wave, wave.enemies.len()));
  assert_eq!(state.remaining, 2);
}

#[test]
fn wave_state_uses_first_enemy_spawn_interval() {
  let wave = Wave {
    enemies: vec![(EnemyType::Green, Duration::from_secs(5), vec![])],
    current: 0,
  };
  let state = WaveState::from((&wave, wave.enemies.len()));
  assert_eq!(state.enemy_spawn_timer.duration(), Duration::from_secs(5));
}

#[test]
fn wave_state_waits_ten_seconds_between_waves() {
  let wave = Wave {
    enemies: vec![(EnemyType::Green, Duration::from_secs(5), vec![])],
    current: 0,
  };
  let state = WaveState::from((&wave, wave.enemies.len()));
  assert_eq!(state.wave_spawn_timer.duration(), Duration::from_secs(10));
}

#[test]
fn default_wave_has_no_enemies() {
  assert!(Wave::default().enemies.is_empty());
}

#[test]
fn default_wave_starts_at_first_enemy() {
  assert_eq!(Wave::default().current, 0);
}

#[test]
fn default_route_state_starts_at_zero() {
  assert_eq!(RouteState::default().next_route, 0);
}

#[test]
fn only_one_lane_open_on_first_wave() {
  assert_eq!(lanes_open(0, 3), 1);
}

#[test]
fn only_one_lane_open_just_before_threshold() {
  assert_eq!(lanes_open(1, 3), 1);
}

#[test]
fn all_lanes_open_at_threshold() {
  assert_eq!(lanes_open(2, 3), 3);
}

#[test]
fn all_lanes_open_after_threshold() {
  assert_eq!(lanes_open(5, 3), 3);
}

#[test]
fn single_lane_map_never_opens_a_second() {
  assert_eq!(lanes_open(9, 1), 1);
}

#[test]
fn round_robin_picks_first_lane_at_start() {
  assert_eq!(pick_route(0, 2), 0);
}

#[test]
fn round_robin_picks_second_lane_next() {
  assert_eq!(pick_route(1, 2), 1);
}

#[test]
fn round_robin_wraps_back_to_first_lane() {
  assert_eq!(pick_route(2, 2), 0);
}

#[test]
fn round_robin_wraps_back_to_second_lane() {
  assert_eq!(pick_route(3, 2), 1);
}

#[test]
fn round_robin_with_one_lane_always_picks_it() {
  assert_eq!(pick_route(7, 1), 0);
}

#[test]
fn round_robin_across_three_lanes() {
  assert_eq!(pick_route(5, 3), 2);
}
