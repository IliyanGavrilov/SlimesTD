use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::enemy::*;
use crate::{GameState, Tower};
use crate::movement::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Bullet>()
      .add_systems((despawn_bullets, bullet_enemy_collision)
        .in_set(OnUpdate(GameState::Gameplay)));
  }
}

#[derive(Bundle)]
pub struct BulletBundle {
  pub bullet: Bullet,
  pub movement: Movement,
  pub sprite: SpriteBundle,
  pub name: Name
}

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
  bullets: Query<(Entity, &Bullet, &Parent, &GlobalTransform)>,
  mut enemies: Query<(&mut Enemy, &Transform)>,
  mut towers: Query<&mut Tower>
) {
  for (
    bullet_entity,
    bullet,
    tower_parent,
    bullet_transform) in &bullets {
    for (mut enemy, enemy_transform) in &mut enemies {
      if collide(bullet_transform.translation(), Vec2::new(40., 22.),
                 enemy_transform.translation, Vec2::new(30., 30.)).is_some() {
        // Update tower's total damage
        let mut tower = towers.get_mut(tower_parent.get()).unwrap();
        if enemy.health >= bullet.damage as i32 {
          tower.total_damage += bullet.damage;
        }
        else {
          tower.total_damage += enemy.health as u32;
        }
        
        // Despawn bullet upon hit and damage enemy
        commands.entity(bullet_entity).despawn_recursive();
        enemy.health -= bullet.damage as i32;
        break;
      }
    }
  }
}