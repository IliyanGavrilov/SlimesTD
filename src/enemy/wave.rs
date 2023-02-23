use std::time::Duration;
use bevy::prelude::*;
use serde::Deserialize;
use std::fs::File;

use crate::assets::*;
use crate::enemy::*;
use crate::map::*;
use crate::GameState;

pub struct WavePlugin;

impl Plugin for WavePlugin {
  fn build(&self, app: &mut App) {
    app.add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(spawn_waves))
      .add_startup_system_to_stage(StartupStage::PreStartup, load_waves);
  }
}

#[derive(Resource, Default, Deserialize)]
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

#[derive(Component, Deserialize)]
pub struct Wave {
  pub enemies: Vec<(EnemyType, Duration)>,
  pub current: usize // Current enemy
}

impl Default for Wave {
  fn default() -> Self {
    Self {
      enemies: vec![],
      current: 0
    }
  }
}

#[derive(Resource)]
pub struct WaveState {
  pub wave_spawn_timer: Timer,
  pub enemy_spawn_timer: Timer,
  pub remaining: usize
}

impl From<(&Wave, usize)> for WaveState {
  fn from(wave: (&Wave, usize)) -> Self {
    Self {
      wave_spawn_timer: Timer::new(Duration::from_secs(10), TimerMode::Once),
      enemy_spawn_timer: Timer::new(wave.0.enemies[wave.0.current].1,
                                    TimerMode::Repeating),
      remaining: wave.1
    }
  }
}

fn spawn_waves(
  mut commands: Commands,
  assets: Res<GameAssets>, // Tower and enemy assets
  map_path: Res<MapPath>,
  mut waves: ResMut<Waves>,
  mut wave_state: ResMut<WaveState>,
  enemy_stats: Res<EnemyTypeStats>,
  time: Res<Time>
) {
  // If all enemies in wave have finished, if button has been pressed
  // or if in-between waves timer has finished !!!
  if wave_state.remaining == 0 {
    wave_state.wave_spawn_timer.tick(time.delta());
    if !wave_state.wave_spawn_timer.just_finished() {
      return;
    }
    if let Some(next_wave) = waves.advance() {
      commands.insert_resource(WaveState::from((next_wave, next_wave.enemies.len())));
    }
  }
  
  let Some(current_wave) = waves.current() else {
    return;
  };
  
  wave_state.enemy_spawn_timer.tick(time.delta());
  if !wave_state.enemy_spawn_timer.just_finished() {
    return;
  }
  
  let index = current_wave.enemies.len() - wave_state.remaining;
  
  spawn_enemy(&mut commands,
              &map_path,
              current_wave.enemies[index].0,
              &assets,
              map_path.checkpoints[0],
              Path { index: 0 },
              &enemy_stats);
  
  wave_state.enemy_spawn_timer = Timer::new(
    current_wave.enemies[index].1,
    TimerMode::Repeating);
  
  wave_state.remaining -= 1;
}

fn load_waves(
  mut commands: Commands
) {
  let f = File::open("./assets/game_data/waves.ron").expect("Failed opening wave file!");
  let waves: Waves = match ron::de::from_reader(f) {
    Ok(x) => x,
    Err(e) => {
      info!("Failed to load waves: {}", e);
      
      std::process::exit(1);
    }
  };
  
  let num_enemies = waves.waves[0].enemies.len();
  
  
  commands.insert_resource(waves);
  commands.insert_resource(WaveState {
    wave_spawn_timer: Timer::new(Duration::from_secs(10), TimerMode::Once),
    enemy_spawn_timer: Timer::new(Duration::from_millis(1),
                                  TimerMode::Repeating),
    remaining: num_enemies
  })
}