use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::Deserialize;
use std::time::Duration;

use crate::assets::*;
use crate::enemy::*;
use crate::map::*;
use crate::{GameData, GameState};

pub struct WavePlugin;

impl Plugin for WavePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<WaveClearedEvent>()
      .add_system(load_waves.in_schedule(OnExit(GameState::AssetLoading)))
      .add_system(spawn_waves.in_set(OnUpdate(GameState::Gameplay)));
  }
}

pub struct WaveClearedEvent {
  pub index: usize,
}

#[derive(Resource, Default, Deserialize, TypeUuid)]
#[uuid = "2ee4097e-4768-40d6-962b-e7ad0b750219"]
pub struct Waves {
  pub waves: Vec<Wave>,
  pub current: usize,
}

impl Waves {
  pub fn current(&self) -> Option<&Wave> {
    return self.waves.get(self.current);
  }

  pub fn advance(
    &mut self,
    wave_cleared_writer: &mut EventWriter<WaveClearedEvent>,
  ) -> Option<&Wave> {
    wave_cleared_writer.send(WaveClearedEvent {
      index: self.current,
    });
    self.current += 1;
    return self.current();
  }
}

#[derive(Component, Deserialize)]
pub struct Wave {
  pub enemies: Vec<(EnemyType, Duration)>,
  pub current: usize, // Current enemy
}

impl Default for Wave {
  fn default() -> Self {
    Self {
      enemies: vec![],
      current: 0,
    }
  }
}

#[derive(Resource)]
pub struct WaveState {
  pub wave_spawn_timer: Timer,
  pub enemy_spawn_timer: Timer,
  pub remaining: usize,
}

impl From<(&Wave, usize)> for WaveState {
  fn from(wave: (&Wave, usize)) -> Self {
    Self {
      wave_spawn_timer: Timer::new(Duration::from_secs(10), TimerMode::Once),
      enemy_spawn_timer: Timer::new(wave.0.enemies[wave.0.current].1, TimerMode::Repeating),
      remaining: wave.1,
    }
  }
}

fn spawn_waves(
  mut commands: Commands,
  assets: Res<GameAssets>, // Tower and enemy assets
  game_data: Res<GameData>,
  map: Res<Assets<Map>>,
  mut waves: ResMut<Assets<Waves>>,
  mut wave_state: ResMut<WaveState>,
  enemy_type_assets: Res<Assets<EnemyTypeStats>>,
  time: Res<Time>,
  mut wave_cleared_writer: EventWriter<WaveClearedEvent>,
) {
  let Some(map_path) = map.get(&game_data.map)
    else { return; };
  let Some(waves) = waves.get_mut(&game_data.enemy_waves)
    else { return; };

  // If all enemies in wave have finished, if button has been pressed
  // or if in-between waves timer has finished !!!
  if wave_state.remaining == 0 {
    wave_state.wave_spawn_timer.tick(time.delta());
    if !wave_state.wave_spawn_timer.just_finished() {
      return;
    }
    if let Some(next_wave) = waves.advance(&mut wave_cleared_writer) {
      wave_state.remaining = next_wave.enemies.len(); // !!!
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
  //if wave_state.remaining > 0 { // !!!
  let index = current_wave.enemies.len() - wave_state.remaining;
  //println!("Enemy #{}", (current_wave.enemies.len() - wave_state.remaining + 1));

  let Some(enemy_stats) = enemy_type_assets.get(&game_data.enemy_type_stats)
    else { return; };

  spawn_enemy(
    &mut commands,
    &map_path,
    current_wave.enemies[index].0,
    &assets,
    map_path.checkpoints[0],
    Path { index: 0 },
    enemy_stats,
  );

  wave_state.enemy_spawn_timer = Timer::new(current_wave.enemies[index].1, TimerMode::Repeating);

  wave_state.remaining -= 1;
  //}
}

fn load_waves(mut commands: Commands, game_data: Res<GameData>, waves: Res<Assets<Waves>>) {
  let Some(waves) = waves.get(&game_data.enemy_waves)
    else { return; };

  let num_enemies = waves.waves[0].enemies.len();

  commands.insert_resource(WaveState {
    wave_spawn_timer: Timer::new(Duration::from_secs(10), TimerMode::Once),
    enemy_spawn_timer: Timer::new(Duration::from_millis(1), TimerMode::Repeating),
    remaining: num_enemies,
  });
}
