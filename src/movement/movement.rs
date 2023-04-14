use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{Bullet, GameState};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Movement>()
      .add_system(basic_movement.in_set(OnUpdate(GameState::Gameplay)));
  }
}

#[derive(Reflect, Debug, Component, Clone, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Movement {
  pub direction: Vec3,
  pub speed: f32,
  pub distance_travelled: f32
}

impl Movement {
  pub fn new(direction: Vec3, speed: f32) -> Self {
    Self {
      direction,
      speed,
      distance_travelled: 0.
    }
  }
}

fn basic_movement(
  mut entities: Query<(&mut Movement, &mut Transform), With<Bullet>>,
  time: Res<Time>
) {
  for (mut movement, mut transform) in &mut entities {
    let distance = movement.direction.normalize() * movement.speed * time.delta_seconds();
    movement.distance_travelled += distance.length();
    transform.translation += distance;
  }
}