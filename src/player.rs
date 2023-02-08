use bevy::prelude::*;
use crate::{EnemyDeathEvent, GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Player>()
      .add_system_set(SystemSet::on_enter(GameState::Gameplay)
        .with_system(spawn_player))
      .add_system_set(
        SystemSet::on_update(GameState::Gameplay)
          .with_system(give_money_on_enemy_death));
  }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
  pub money: i32,
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