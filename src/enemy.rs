use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Enemy>()
       .add_system(despawn_enemy_on_death);
  }
}

#[derive(Bundle)]
pub struct EnemyBundle {

}

impl Default for EnemyBundle {
  fn default() -> Self {
    Self {
    }
  }
}

// !!! Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy {
  pub health: i32
}

fn despawn_enemy_on_death(mut commands: Commands, enemies: Query<(Entity, &mut Enemy)>) {
  for (entity, enemy) in &enemies {
    if enemy.health <= 0 {
      commands.entity(entity).despawn_recursive();
    }
  }
}