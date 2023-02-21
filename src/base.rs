use bevy::prelude::*;
use crate::GameState;

pub struct BasePlugin;

impl Plugin for BasePlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Base>()
      .add_system_set(SystemSet::on_enter(GameState::Gameplay)
        .with_system(spawn_base));
  }
}

// !!! Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Base {
  pub health: i32
}

fn spawn_base(mut commands: Commands) {
  commands.spawn((Base { health: 100 }, Name::new("Base")));
}

pub fn damage_base(
  commands: &mut Commands,
  entity: &Entity,
  enemy_health: i32,
  base: &mut Base
) {
  commands.entity(*entity).despawn_recursive();
  
  if base.health > 0 {
    base.health -= enemy_health as i32;
  }
  
  if base.health <= 0{
    base.health = 0;
    info!("GAME OVER");
  }
}