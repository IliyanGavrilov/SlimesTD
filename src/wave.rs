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

#[derive(Resource)]
pub struct Waves {
  pub waves: Vec<Wave>
}

#[derive(Component)]
pub struct Wave {
  pub enemies: Vec<(EnemyType, Duration)>
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
  waves: Res<Waves>,
  time: Res<Time>,
  wave_enemies: Query<With<WaveIndex>>
) {
  // If all enemies in wave have finished, if button has been pressed
  // or if in-between waves timer has finished !!!
  if wave_enemies.is_empty() {
    for (index, wave) in waves.waves.iter().enumerate() {
      for enemy in &wave.enemies {
        spawn_enemy(&mut commands,
                    &map_path,
                    enemy.0,
                    &assets,
                    map_path.checkpoints[0],
                    Path { index: 0 },
                    WaveIndex { index: index + 1});
      }
    }
  }
}

fn load_waves(
  mut commands: Commands
) {
  commands.insert_resource(Waves {
    waves: vec![
      Wave {
        enemies: vec![(EnemyType::Green, Duration::from_millis(500)); 20]
      },
      Wave {
        enemies: vec![(EnemyType::Green, Duration::from_millis(250)); 35]
      },
      Wave {
        enemies: vec![(EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(500)),
                      (EnemyType::Yellow, Duration::from_millis(500)),
                      (EnemyType::Yellow, Duration::from_millis(500)),
                      (EnemyType::Yellow, Duration::from_millis(500)),
                      (EnemyType::Yellow, Duration::from_millis(500)),
                      (EnemyType::Yellow, Duration::from_millis(500)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250)),
                      (EnemyType::Green, Duration::from_millis(250))
        ]
      },
    ]
  })
}