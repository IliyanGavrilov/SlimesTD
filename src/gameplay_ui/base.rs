use crate::GameState;
use bevy::prelude::*;

pub struct BasePlugin;

impl Plugin for BasePlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Base>()
      .add_system(spawn_base.in_schedule(OnEnter(GameState::Gameplay)));
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
    info!("GAME OVER");
  }
}
