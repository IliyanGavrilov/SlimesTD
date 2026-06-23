use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use serde::{Deserialize, Serialize};

use crate::enemy::*;
use crate::movement::*;
use crate::{
  EnemyHitEvent, FarmBehavior, FarmTower, FloatingTextEvent, GameState, Player, Slowed, Tower,
  game_not_paused,
};

/// An on-hit combat effect carried by a projectile. Analogous to `FarmBehavior`
/// for income: data on the component, applied by systems. Configured per tower in
/// `tower_stats.ron`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OnHitEffect {
  /// Reduce the enemy's speed to `factor` (0..1) of normal for `duration` seconds.
  Slow { factor: f32, duration: f32 },
  /// Halt the enemy entirely for `duration` seconds (a slow with factor 0).
  Stun { duration: f32 },
  /// Deal `damage` to every other enemy within `radius` of the impact point.
  Splash { radius: f32, damage: u32 },
}

/// Requests area damage around an impact point (sent by Splash hits).
pub struct SplashEvent {
  pub position: Vec3,
  pub radius: f32,
  pub damage: u32,
  /// The directly-hit enemy, excluded so it isn't double-damaged.
  pub exclude: Entity,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Bullet>()
      .add_event::<SplashEvent>()
      .add_systems(
        (
          despawn_bullets.run_if(game_not_paused),
          bullet_enemy_collision.run_if(game_not_paused),
          apply_splash.after(bullet_enemy_collision).run_if(game_not_paused),
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
  #[reflect(ignore)]
  pub effect: Option<OnHitEffect>,
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
  mut enemies: Query<(Entity, &mut Enemy, &Transform)>,
  mut towers: Query<(&mut Tower, &GlobalTransform)>,
  farm_towers: Query<&FarmTower>,
  mut player: Query<&mut Player>,
  mut hit_writer: EventWriter<EnemyHitEvent>,
  mut float_writer: EventWriter<FloatingTextEvent>,
  mut splash_writer: EventWriter<SplashEvent>,
) {
  for (bullet_entity, mut bullet, tower_parent, bullet_transform) in &mut bullets {
    for (enemy_entity, mut enemy, enemy_transform) in &mut enemies {
      if collide(
        bullet_transform.translation(),
        Vec2::new(40., 22.),
        enemy_transform.translation,
        Vec2::new(30., 30.),
      )
      .is_some()
      {
        let (mut tower, tower_transform) = towers.get_mut(tower_parent.get()).unwrap();
        if enemy.health >= bullet.damage as i32 {
          tower.total_damage += bullet.damage;
        } else {
          tower.total_damage += enemy.health.max(0) as u32;
        }

        enemy.health -= bullet.damage as i32;
        hit_writer.send(EnemyHitEvent { entity: enemy_entity });
        float_writer.send(FloatingTextEvent {
          position: enemy_transform.translation,
          text: format!("-{}", bullet.damage),
          color: Color::WHITE,
        });

        if enemy.health <= 0 {
          if let Ok(farm) = farm_towers.get(tower_parent.get()) {
            if let FarmBehavior::SelfKill { income_per_kill } = &farm.behavior {
              if let Ok(mut p) = player.get_single_mut() {
                p.money += *income_per_kill as usize;
              }
              float_writer.send(FloatingTextEvent {
                position: tower_transform.translation(),
                text: format!("+${}", income_per_kill),
                color: Color::GOLD,
              });
            }
          }
        }

        // Apply the projectile's on-hit effect.
        if let Some(effect) = bullet.effect.clone() {
          match effect {
            // Slow/Stun add a status component; skip if the hit was lethal (the
            // enemy despawns this frame) to avoid inserting onto a dead entity.
            OnHitEffect::Slow { factor, duration } if enemy.health > 0 => {
              commands.entity(enemy_entity).insert(Slowed {
                factor,
                timer: Timer::from_seconds(duration, TimerMode::Once),
              });
            }
            OnHitEffect::Stun { duration } if enemy.health > 0 => {
              commands.entity(enemy_entity).insert(Slowed {
                factor: 0.0,
                timer: Timer::from_seconds(duration, TimerMode::Once),
              });
            }
            OnHitEffect::Splash { radius, damage } => {
              splash_writer.send(SplashEvent {
                position: enemy_transform.translation,
                radius,
                damage,
                exclude: enemy_entity,
              });
            }
            _ => {}
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

fn apply_splash(
  mut events: EventReader<SplashEvent>,
  mut enemies: Query<(Entity, &mut Enemy, &Transform)>,
  mut hit_writer: EventWriter<EnemyHitEvent>,
  mut float_writer: EventWriter<FloatingTextEvent>,
) {
  for splash in events.iter() {
    for (entity, mut enemy, transform) in &mut enemies {
      if entity == splash.exclude || enemy.health <= 0 {
        continue;
      }
      if transform.translation.distance(splash.position) <= splash.radius {
        enemy.health -= splash.damage as i32;
        hit_writer.send(EnemyHitEvent { entity });
        float_writer.send(FloatingTextEvent {
          position: transform.translation,
          text: format!("-{}", splash.damage),
          color: Color::ORANGE,
        });
      }
    }
  }
}