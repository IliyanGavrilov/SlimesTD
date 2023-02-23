use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::GameState;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Movement>()
      .add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(basic_movement));
  }
}

#[derive(Reflect, Component, Clone, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Movement {
  pub direction: Vec3,
  pub speed: f32
}

fn basic_movement(mut entities: Query<(&Movement, &mut Transform)>, time: Res<Time>) {
  for (movement, mut transform) in &mut entities {
    transform.translation += movement.direction.normalize() * movement.speed * time.delta_seconds();
  }
}