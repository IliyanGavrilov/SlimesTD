use bevy::prelude::*;

use crate::{GameState, EnemyDeathEvent, WaveClearedEvent};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Player>()
      .add_system_set(SystemSet::on_enter(GameState::Gameplay)
        .with_system(spawn_player))
      .add_system_set(
        SystemSet::on_update(GameState::Gameplay)
          .with_system(give_money_on_enemy_death)
          .with_system(give_money_on_wave_cleared));
  }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    pub money: usize,
}

fn spawn_player(mut commands: Commands) {
  commands.spawn((Player { money: 100 }, Name::new("Player")));
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