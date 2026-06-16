use bevy::prelude::*;

use crate::{EnemyDeathEvent, GameDifficulty, GameState, WaveClearedEvent, game_not_paused};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Player>()
      .add_system(spawn_player.in_schedule(OnEnter(GameState::Gameplay)))
      .add_systems(
        (
          give_money_on_enemy_death.run_if(game_not_paused),
          give_money_on_wave_cleared.run_if(game_not_paused),
        )
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_system(cleanup_player.in_schedule(OnExit(GameState::Gameplay)));
  }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
  pub money: usize,
}

fn spawn_player(mut commands: Commands, difficulty: Res<GameDifficulty>) {
  let starting_money = match *difficulty {
    GameDifficulty::Normal => 100,
    GameDifficulty::Test => 5000,
  };
  commands.spawn((Player { money: starting_money }, Name::new("Player")));
}

fn cleanup_player(mut commands: Commands, players: Query<Entity, With<Player>>) {
  for entity in &players {
    commands.entity(entity).despawn_recursive();
  }
}

fn give_money_on_enemy_death(
  mut player: Query<&mut Player>,
  mut death_events: EventReader<EnemyDeathEvent>,
) {
  let mut player = player.single_mut();
  for _ in death_events.iter() {
    player.money += 10;
  }
}

fn give_money_on_wave_cleared(
  mut player: Query<&mut Player>,
  mut wave_events: EventReader<WaveClearedEvent>,
) {
  let mut player = player.single_mut();
  for wave in wave_events.iter() {
    player.money += wave.index + 101;
  }
}
