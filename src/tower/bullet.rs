use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::enemy::*;
use crate::movement::*;
use crate::{FarmBehavior, FarmTower, GameState, Player, Tower, game_not_paused};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Bullet>()
      .add_systems(
        (
          despawn_bullets.run_if(game_not_paused),
          bullet_enemy_collision.run_if(game_not_paused),
        )
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_system(cleanup_bullets.in_schedule(OnExit(GameState::Gameplay)));
  }
}

#[derive(Bundle)]
pub struct BulletBundle {
  pub bullet: Bullet,
  pub movement: Movement,
  pub sprite: SpriteBundle,
  pub name: Name,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
  pub damage: u32,
  pub pierce_remaining: u32,
  pub lifetime: Timer,
}

fn cleanup_bullets(mut commands: Commands, bullets: Query<Entity, With<Bullet>>) {
  for entity in &bullets {
    commands.entity(entity).despawn_recursive();
  }
}

fn despawn_bullets(
  mut commands: Commands,
  mut bullets: Query<(Entity, &mut Bullet)>,
  time: Res<Time>,
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
  mut bullets: Query<(Entity, &mut Bullet, &Parent, &GlobalTransform)>,
  mut enemies: Query<(&mut Enemy, &Transform)>,
  mut towers: Query<&mut Tower>,
  farm_towers: Query<&FarmTower>,
  mut player: Query<&mut Player>,
) {
  for (bullet_entity, mut bullet, tower_parent, bullet_transform) in &mut bullets {
    for (mut enemy, enemy_transform) in &mut enemies {
      if collide(
        bullet_transform.translation(),
        Vec2::new(40., 22.),
        enemy_transform.translation,
        Vec2::new(30., 30.),
      )
      .is_some()
      {
        let mut tower = towers.get_mut(tower_parent.get()).unwrap();
        if enemy.health >= bullet.damage as i32 {
          tower.total_damage += bullet.damage;
        } else {
          tower.total_damage += enemy.health.max(0) as u32;
        }

        enemy.health -= bullet.damage as i32;

        if enemy.health <= 0 {
          if let Ok(farm) = farm_towers.get(tower_parent.get()) {
            if let FarmBehavior::SelfKill { income_per_kill } = &farm.behavior {
              if let Ok(mut p) = player.get_single_mut() {
                p.money += *income_per_kill as usize;
              }
            }
          }
        }

        if bullet.pierce_remaining > 0 {
          bullet.pierce_remaining -= 1;
        } else {
          commands.entity(bullet_entity).despawn_recursive();
        }
        break;
      }
    }
  }
}