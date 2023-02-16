use std::time::Duration;
use bevy::prelude::*;
use crate::{EnemyType, GameAssets, GameState, MapPath, Path, spawn_enemy};

pub struct WavePlugin;

impl Plugin for WavePlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<WaveIndex>()
      .add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(spawn_waves))
      .add_startup_system_to_stage(StartupStage::PreStartup, load_waves);
  }
}

#[derive(Resource, Default)]
pub struct Waves {
  pub waves: Vec<Wave>,
  pub current: usize
}

impl Waves {
  pub fn current(&self) -> Option<&Wave> {
    return self.waves.get(self.current);
  }
  pub fn advance(&mut self) -> Option<&Wave> {
    self.current += 1;
    return self.current();
  }
}

#[derive(Component)]
pub struct Wave {
  pub enemies: Vec<(EnemyType, Duration)>,
  pub current: usize, // Current enemy
  pub num: usize // Number of remaining enemies to spawn
}

impl Default for Wave {
  fn default() -> Self {
    Self {
      enemies: vec![],
      current: 0,
      num: 0
    }
  }
}

#[derive(Resource)]
pub struct WaveState {
  pub wave_timer: Timer,
  pub spawn_timer: Timer,
  pub remaining: usize
}

impl From<&Wave> for WaveState {
  fn from(wave: &Wave) -> Self {
    Self {
      wave_timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
      spawn_timer: Timer::new(wave.enemies[wave.current].1, TimerMode::Repeating),
      remaining: wave.num
    }
  }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct WaveIndex {
  pub index: usize
}

fn spawn_waves(
  mut commands: Commands,
  assets: Res<GameAssets>, // Tower and enemy assets
  map_path: Res<MapPath>,
  mut waves: ResMut<Waves>,
  mut wave_state: ResMut<WaveState>,
  time: Res<Time>
) {
  // If all enemies in wave have finished, if button has been pressed
  // or if in-between waves timer has finished !!!
  if wave_state.remaining == 0 {
    wave_state.wave_timer.tick(time.delta());
    if !wave_state.wave_timer.just_finished() {
      return;
    }
    if let Some(next_wave) = waves.advance() {
      commands.insert_resource(WaveState::from(next_wave));
    }
  }
  
  let Some(current_wave) = waves.current() else {
    return;
  };
  
  wave_state.spawn_timer.tick(time.delta());
  if !wave_state.spawn_timer.just_finished() {
    return;
  }
  
  let index = current_wave.enemies.len() - wave_state.remaining;
  
  spawn_enemy(&mut commands,
              &map_path,
              current_wave.enemies[index].0,
              &assets,
              map_path.checkpoints[0],
              Path { index: 0 },
              WaveIndex { index: waves.current + 1}); // !!!
  
  wave_state.spawn_timer = Timer::new(
    current_wave.enemies[index].1,
    TimerMode::Repeating);
  
  wave_state.remaining -= 1;
}

fn load_waves(
  mut commands: Commands
) {
  let first_wave_num_enemies = 20;
  commands.insert_resource(Waves {
    waves: vec![
      Wave {
        enemies: vec![(EnemyType::Green, Duration::from_millis(1500)); 20],
        num: first_wave_num_enemies,
        ..default()
      },
      Wave {
        enemies: vec![(EnemyType::Green, Duration::from_millis(1000)); 35],
        num: 35,
        ..default()
      },
      Wave {
        enemies: vec![(EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(2000)),
                      (EnemyType::Yellow, Duration::from_millis(2000)),
                      (EnemyType::Yellow, Duration::from_millis(2000)),
                      (EnemyType::Yellow, Duration::from_millis(2000)),
                      (EnemyType::Yellow, Duration::from_millis(2000)),
                      (EnemyType::Yellow, Duration::from_millis(2000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000)),
                      (EnemyType::Green, Duration::from_millis(1000))
        ],
        num: 30,
        ..default()
      },
      Wave {
        enemies: vec![(EnemyType::Red, Duration::from_millis(500)); 20],
        num: 20,
        ..default()
      }
    ],
    ..default()
  });
  commands.insert_resource(WaveState {
    wave_timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
    spawn_timer: Timer::new(Duration::from_millis(1), TimerMode::Repeating),
    remaining: first_wave_num_enemies
  })
}