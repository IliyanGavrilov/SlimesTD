use crate::{GameState, game_not_paused};
use bevy::prelude::*;

pub struct BasePlugin;

impl Plugin for BasePlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Base>()
      .add_system(spawn_base.in_schedule(OnEnter(GameState::Gameplay)))
      .add_system(check_game_over.in_set(OnUpdate(GameState::Gameplay)).run_if(game_not_paused))
      .add_system(cleanup_base.in_schedule(OnExit(GameState::Gameplay)));
  }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Base {
  pub health: i32,
}

fn spawn_base(mut commands: Commands) {
  commands.spawn((Base { health: 100 }, Name::new("Base")));
}

pub fn damage_base(commands: &mut Commands, entity: &Entity, enemy_health: i32, base: &mut Base) {
  commands.entity(*entity).despawn_recursive();

  if base.health > enemy_health {
    base.health -= enemy_health;
  } else {
    base.health = 0;
  }
}

fn check_game_over(base: Query<&Base>, mut next_state: ResMut<NextState<GameState>>) {
  if let Ok(base) = base.get_single() {
    if base.health <= 0 {
      next_state.set(GameState::GameOver);
    }
  }
}

fn cleanup_base(mut commands: Commands, bases: Query<Entity, With<Base>>) {
  for entity in &bases {
    commands.entity(entity).despawn_recursive();
  }
}

#[cfg(test)]
#[path = "gameplay_ui/base_tests.rs"]
mod tests;