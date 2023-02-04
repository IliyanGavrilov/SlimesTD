use bevy::prelude::*;
pub use crate::{GameAssets, Movement, enemy_type::EnemyType};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Enemy>()
       .add_system(despawn_enemy_on_death);
  }
}

// #[derive(Bundle)]
// pub struct EnemyBundle {
//   pub enemy: Enemy,
//   pub movement: Movement,
//   pub animation_indices: AnimationIndices,
//
// }
//
// impl Default for EnemyBundle {
//   fn default() -> Self {
//     Self {
//     }
//   }
// }

// !!! Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy {
  pub health: i32
}

impl Enemy {
  pub fn new(health: i32) -> Self {
    Self {
      health
    }
  }
}

pub fn spawn_enemy(
  commands: &mut Commands,
  enemy_type: EnemyType,
  assets: &GameAssets,
  position: Vec3,
  direction: Vec3
) {
  // Enemy, Movement, AnimationIndices, AnimationTimer
  let (enemy,
       enemy_movement,
       enemy_animation_indices,
       enemy_animation_timer) = enemy_type.get_enemy(direction);
  // Tower
  commands.spawn(SpriteSheetBundle {
    texture_atlas: assets.enemy.clone(),
    transform: Transform::from_translation(position),
    sprite: TextureAtlasSprite::new(enemy_animation_indices.first),
    ..default()
  }
  )
    .insert(enemy_type)
    .insert(enemy)
    .insert(enemy_movement)
    .insert(enemy_animation_indices)
    .insert(enemy_animation_timer)
    .insert(Name::new(format!("{enemy_type}_Enemy"))); // !!! Debug
}

fn despawn_enemy_on_death(mut commands: Commands, enemies: Query<(Entity, &mut Enemy)>) {
  for (entity, enemy) in &enemies {
    if enemy.health <= 0 {
      commands.entity(entity).despawn_recursive();
    }
  }
}