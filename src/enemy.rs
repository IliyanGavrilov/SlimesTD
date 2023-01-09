use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Enemy>()
       .add_system(move_enemies)
       .add_system(despawn_enemy_on_death);
  }
}

// !!! Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy { // !!! Split health
  pub health: i32,
  pub speed: f32
}

fn move_enemies(mut enemies: Query<(&Enemy, &mut Transform)>, time: Res<Time>) {
  for (enemy, mut transform) in &mut enemies {
    transform.translation.y += enemy.speed * time.delta_seconds();
  }
}

fn despawn_enemy_on_death(mut commands: Commands, enemies: Query<(Entity, &mut Enemy)>) {
  for (entity, enemy) in &enemies {
    if enemy.health <= 0 {
      commands.entity(entity).despawn_recursive();
    }
  }
}