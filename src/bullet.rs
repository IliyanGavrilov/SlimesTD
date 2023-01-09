use bevy::prelude::*;

pub struct BulletPlugin;

pub use crate::enemy::*;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Bullet>()
       .add_system(move_bullets)
       .add_system(despawn_bullets)
       .add_system(bullet_enemy_collision);
  }
}

// !!! Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
  pub direction: Vec3,
  pub speed: f32,
  pub damage: i32,
  pub lifetime: Timer // !!! fix?
}

fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
  for (bullet, mut transform) in &mut bullets {
    transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
  }
}

fn despawn_bullets(
  mut commands: Commands,
  mut bullets: Query<(Entity, &mut Bullet)>,
  time: Res<Time>
) {
  for (entity, mut bullet) in &mut bullets {
    bullet.lifetime.tick(time.delta());
    // If the lifetime timer finished, despawn bullet
    if bullet.lifetime.finished() {
      // Despawn entities and their children
      commands.entity(entity).despawn_recursive()
    }
  }
}

fn bullet_enemy_collision(
  mut commands: Commands,
  bullets: Query<(Entity, &Bullet, &GlobalTransform)>,
  mut enemies: Query<(&mut Enemy, &Transform)>
) {
  for (bullet_entity, bullet, bullet_transform) in &bullets {
    for (mut enemy, enemy_transform) in &mut enemies {
      if Vec3::distance(bullet_transform.translation(), enemy_transform.translation) < 20. {
        commands.entity(bullet_entity).despawn_recursive();
        enemy.health -= bullet.damage;
        break;
      }
    }
  }
}