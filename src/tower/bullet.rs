use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use serde::{Deserialize, Serialize};

use crate::enemy::*;
use crate::movement::*;
use crate::{
  game_not_paused, is_targetable, ChainBoltEvent, EnemyHitEvent, FarmBehavior, FarmTower,
  FloatingTextEvent, GameState, KnockedBack, Player, Slowed, Tower,
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
  /// Damage-over-time: `dps` damage each second for `duration` seconds.
  Poison { dps: u32, duration: f32 },
  /// Lightning that arcs to up to `jumps` nearby enemies (each within `radius`
  /// of the previous), dealing `damage` to each.
  Chain {
    jumps: u32,
    radius: f32,
    damage: u32,
  },
  /// Shove the enemy backwards along its path at `strength` units/sec briefly.
  Knockback { strength: f32 },
}

/// Requests a lightning chain starting from a hit point.
pub struct ChainEvent {
  pub from: Vec3,
  pub jumps: u32,
  pub radius: f32,
  pub damage: u32,
  /// The directly-hit enemy, excluded as a chain target.
  pub exclude: Entity,
  // Whether the firing tower can arc to invisible enemies.
  pub can_see_invisible: bool,
}

/// Requests area damage around an impact point (sent by Splash hits).
pub struct SplashEvent {
  pub position: Vec3,
  pub radius: f32,
  pub damage: u32,
  /// The directly-hit enemy, excluded so it isn't double-damaged.
  pub exclude: Entity,
  // Whether the firing tower can damage invisible enemies.
  pub can_see_invisible: bool,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Bullet>()
      .add_event::<SplashEvent>()
      .add_event::<ChainEvent>()
      .add_systems(
        (
          despawn_bullets.run_if(game_not_paused),
          bullet_enemy_collision.run_if(game_not_paused),
          apply_splash
            .after(bullet_enemy_collision)
            .run_if(game_not_paused),
          apply_chain
            .after(bullet_enemy_collision)
            .run_if(game_not_paused),
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
  mut chain_writer: EventWriter<ChainEvent>,
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
        hit_writer.send(EnemyHitEvent {
          entity: enemy_entity,
          position: enemy_transform.translation,
        });
        float_writer.send(FloatingTextEvent {
          position: enemy_transform.translation,
          text: format!("-{}", bullet.damage),
          color: Color::RED,
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
                can_see_invisible: tower.can_see_invisible,
              });
            }
            OnHitEffect::Poison { dps, duration } if enemy.health > 0 => {
              commands
                .entity(enemy_entity)
                .insert(Poisoned::new(dps, duration));
            }
            OnHitEffect::Chain {
              jumps,
              radius,
              damage,
            } => {
              chain_writer.send(ChainEvent {
                from: enemy_transform.translation,
                jumps,
                radius,
                damage,
                exclude: enemy_entity,
                can_see_invisible: tower.can_see_invisible,
              });
            }
            OnHitEffect::Knockback { strength } if enemy.health > 0 => {
              commands
                .entity(enemy_entity)
                .insert(KnockedBack::new(strength));
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
  mut enemies: Query<(Entity, &mut Enemy, &Transform, Option<&Invisible>)>,
  mut hit_writer: EventWriter<EnemyHitEvent>,
  mut float_writer: EventWriter<FloatingTextEvent>,
) {
  for splash in events.iter() {
    for (entity, mut enemy, transform, invisible) in &mut enemies {
      if entity == splash.exclude
        || enemy.health <= 0
        || !is_targetable(invisible.is_some(), splash.can_see_invisible)
      {
        continue;
      }
      if transform.translation.distance(splash.position) <= splash.radius {
        enemy.health -= splash.damage as i32;
        hit_writer.send(EnemyHitEvent {
          entity,
          position: transform.translation,
        });
        float_writer.send(FloatingTextEvent {
          position: transform.translation,
          text: format!("-{}", splash.damage),
          color: Color::ORANGE,
        });
      }
    }
  }
}

fn apply_chain(
  mut events: EventReader<ChainEvent>,
  mut enemies: Query<(Entity, &mut Enemy, &Transform, Option<&Invisible>)>,
  mut hit_writer: EventWriter<EnemyHitEvent>,
  mut float_writer: EventWriter<FloatingTextEvent>,
  mut bolt_writer: EventWriter<ChainBoltEvent>,
) {
  for chain in events.iter() {
    let mut from = chain.from;
    let mut visited = vec![chain.exclude];

    for _ in 0..chain.jumps {
      // Nearest living, unvisited enemy within radius of the previous target.
      let mut best: Option<(Entity, f32, Vec3)> = None;
      for (entity, enemy, transform, invisible) in enemies.iter() {
        if enemy.health <= 0
          || visited.contains(&entity)
          || !is_targetable(invisible.is_some(), chain.can_see_invisible)
        {
          continue;
        }
        let dist = transform.translation.distance(from);
        if dist <= chain.radius && best.map_or(true, |(_, b, _)| dist < b) {
          best = Some((entity, dist, transform.translation));
        }
      }

      let Some((entity, _, position)) = best else {
        break;
      };
      if let Ok((_, mut enemy, _, _)) = enemies.get_mut(entity) {
        enemy.health -= chain.damage as i32;
      }
      bolt_writer.send(ChainBoltEvent { from, to: position });
      hit_writer.send(EnemyHitEvent { entity, position });
      float_writer.send(FloatingTextEvent {
        position,
        text: format!("-{}", chain.damage),
        color: Color::YELLOW,
      });
      visited.push(entity);
      from = position;
    }
  }
}
