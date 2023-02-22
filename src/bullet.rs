use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
pub use crate::enemy::*;
use crate::GameState;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Bullet>()
      .add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(despawn_bullets)
        .with_system(bullet_enemy_collision));
  }
}

#[derive(Bundle)]
pub struct BulletBundle {
  pub bullet: Bullet,
  pub movement: Movement,
  pub sprite: SpriteBundle,
  pub name: Name
}

// !!! Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
  pub damage: u32,
  pub lifetime: Timer // !!! fix?
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
      //if Vec3::distance(bullet_transform.translation(),
      //                  enemy_transform.translation) <= 35. {
      if collide(bullet_transform.translation(), Vec2::new(40., 22.),
                 enemy_transform.translation, Vec2::new(50., 50.)).is_some() {
        commands.entity(bullet_entity).despawn_recursive();
        enemy.health -= bullet.damage as i32;
        break;
      }
    }
  }
}