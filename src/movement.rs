use bevy::prelude::*;
use crate::GameState;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Movement>()
      .add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(basic_movement));
  }
}

// !!! Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Movement {
  pub direction: Vec3,
  pub speed: f32
}

fn basic_movement(mut entities: Query<(&Movement, &mut Transform)>, time: Res<Time>) {
  for (entity, mut transform) in &mut entities {
    transform.translation += entity.direction.normalize() * entity.speed * time.delta_seconds();
  }
}