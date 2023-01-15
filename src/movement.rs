use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Movement>()
      // .add_system(move_enemies)
      .add_system(basic_movement);
  }
}

// !!! Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Movement {
  pub direction: Vec3,
  pub speed: f32
}

fn basic_movement(mut bullets: Query<(&Movement, &mut Transform)>, time: Res<Time>) {
  for (entity, mut transform) in &mut bullets {
    transform.translation += entity.direction.normalize() * entity.speed * time.delta_seconds();
  }
}