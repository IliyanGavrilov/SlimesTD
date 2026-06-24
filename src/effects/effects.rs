use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::ui::FocusPolicy;

use crate::{
  Enemy, EnemyDeathEvent, GameAssets, GameState, INVISIBLE_ALPHA, Invisible, KnockedBack,
  MainCamera, Poisoned, Slowed, SplashEvent, Tower, game_not_paused,
};

/// Visual juice: small, asset-free feedback effects (enemy hit flash, ...).
pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<EnemyHitEvent>()
      .add_event::<FloatingTextEvent>()
      .add_event::<BaseDamagedEvent>()
      .add_event::<ChainBoltEvent>()
      .init_resource::<ScreenShake>()
      .add_systems(
        (
          apply_hit_flash.run_if(game_not_paused),
          update_hit_flash.run_if(game_not_paused),
          tint_slowed.run_if(game_not_paused),
        )
          .chain()
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_systems(
        (
          spawn_floating_text.run_if(game_not_paused),
          update_floating_text.run_if(game_not_paused),
        )
          .chain()
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_system(screen_shake.in_set(OnUpdate(GameState::Gameplay)))
      .add_systems(
        (
          spawn_death_pop.run_if(game_not_paused),
          update_death_pop.run_if(game_not_paused),
        )
          .chain()
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_systems(
        (
          spawn_place_poof.run_if(game_not_paused),
          update_place_poof.run_if(game_not_paused),
        )
          .chain()
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_systems(
        (
          spawn_hit_spark.run_if(game_not_paused),
          update_hit_spark.run_if(game_not_paused),
        )
          .chain()
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_systems(
        (
          spawn_splash_ring.run_if(game_not_paused),
          update_splash_ring.run_if(game_not_paused),
        )
          .chain()
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_systems(
        (
          spawn_chain_bolt.run_if(game_not_paused),
          update_chain_bolt.run_if(game_not_paused),
        )
          .chain()
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_system(spawn_damage_flash.in_schedule(OnEnter(GameState::Gameplay)))
      .add_system(update_damage_flash.in_set(OnUpdate(GameState::Gameplay)))
      .add_system(cleanup_death_pops.in_schedule(OnExit(GameState::Gameplay)))
      .add_system(cleanup_hit_sparks.in_schedule(OnExit(GameState::Gameplay)))
      .add_system(cleanup_place_poofs.in_schedule(OnExit(GameState::Gameplay)))
      .add_system(cleanup_floating_text.in_schedule(OnExit(GameState::Gameplay)))
      .add_system(cleanup_splash_rings.in_schedule(OnExit(GameState::Gameplay)))
      .add_system(cleanup_chain_bolts.in_schedule(OnExit(GameState::Gameplay)))
      .add_system(cleanup_damage_flash.in_schedule(OnExit(GameState::Gameplay)));
  }
}

/// Request to spawn a rising, fading world-space label. Unified pipeline used for
/// damage numbers ("-N") and tower income ("+$N") alike.
pub struct FloatingTextEvent {
  pub position: Vec3,
  pub text: String,
  pub color: Color,
}

/// Sent when the base takes damage, to trigger a screen shake.
pub struct BaseDamagedEvent;

/// Sent when a bullet damages an enemy, so effects can react without coupling to
/// the bullet/collision code.
pub struct EnemyHitEvent {
  pub entity: Entity,
  pub position: Vec3,
}

/// Tint applied for a frame when an enemy is hit, then lerped back to white.
const FLASH_COLOR: Color = Color::rgb(1.0, 0.35, 0.35);
const FLASH_SECONDS: f32 = 0.12;

#[derive(Component)]
pub struct HitFlash {
  timer: Timer,
}

fn apply_hit_flash(
  mut commands: Commands,
  mut hits: EventReader<EnemyHitEvent>,
  mut enemies: Query<(&mut TextureAtlasSprite, &Enemy)>,
) {
  for hit in hits.iter() {
    if let Ok((mut sprite, enemy)) = enemies.get_mut(hit.entity) {
      // A lethal hit despawns the enemy this frame (death pop handles it); don't
      // queue an insert on an entity that's about to disappear -> avoids B0003.
      if enemy.health <= 0 {
        continue;
      }
      sprite.color = FLASH_COLOR;
      commands.entity(hit.entity).insert(HitFlash {
        timer: Timer::from_seconds(FLASH_SECONDS, TimerMode::Once),
      });
    }
  }
}

fn update_hit_flash(
  mut commands: Commands,
  time: Res<Time>,
  mut flashing: Query<(Entity, &mut HitFlash, &mut TextureAtlasSprite)>,
) {
  for (entity, mut flash, mut sprite) in &mut flashing {
    flash.timer.tick(time.delta());
    let t = (flash.timer.elapsed_secs() / FLASH_SECONDS).clamp(0., 1.);
    // Lerp tint from FLASH_COLOR back to white.
    sprite.color = lerp_color(FLASH_COLOR, Color::WHITE, t);
    if flash.timer.finished() {
      sprite.color = Color::WHITE;
      commands.entity(entity).remove::<HitFlash>();
    }
  }
}

/// A fading, growing copy of a slime's sprite spawned where it died.
const POP_SECONDS: f32 = 0.3;

#[derive(Component)]
pub struct DeathPop {
  timer: Timer,
  base_scale: Vec3,
}

fn spawn_death_pop(mut commands: Commands, mut deaths: EventReader<EnemyDeathEvent>) {
  for death in deaths.iter() {
    let mut sprite = TextureAtlasSprite::new(death.sprite_index);
    sprite.flip_x = death.flip_x;
    commands.spawn((
      SpriteSheetBundle {
        texture_atlas: death.atlas.clone(),
        sprite,
        transform: death.transform,
        ..default()
      },
      DeathPop {
        timer: Timer::from_seconds(POP_SECONDS, TimerMode::Once),
        base_scale: death.transform.scale,
      },
      Name::new("DeathPop"),
    ));
  }
}

fn update_death_pop(
  mut commands: Commands,
  time: Res<Time>,
  mut pops: Query<(
    Entity,
    &mut DeathPop,
    &mut Transform,
    &mut TextureAtlasSprite,
  )>,
) {
  for (entity, mut pop, mut transform, mut sprite) in &mut pops {
    pop.timer.tick(time.delta());
    let t = (pop.timer.elapsed_secs() / POP_SECONDS).clamp(0., 1.);
    transform.scale = pop.base_scale * (1.0 + 0.6 * t);
    sprite.color.set_a(1.0 - t);
    if pop.timer.finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn cleanup_death_pops(mut commands: Commands, pops: Query<Entity, With<DeathPop>>) {
  for entity in &pops {
    commands.entity(entity).despawn_recursive();
  }
}

/// An expanding, fading ring spawned when a tower is placed.
const POOF_SECONDS: f32 = 0.35;

#[derive(Component)]
pub struct PlacePoof {
  timer: Timer,
  material: Handle<ColorMaterial>,
}

fn spawn_place_poof(
  mut commands: Commands,
  new_towers: Query<&Transform, Added<Tower>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for tower_transform in &new_towers {
    let material = materials.add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 0.6)));
    // Draw above the tower sprite so the ring reads clearly, then fades fast.
    let position = tower_transform.translation.truncate().extend(100.);
    commands.spawn((
      MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(28.).into()).into(),
        material: material.clone(),
        transform: Transform::from_translation(position),
        ..default()
      },
      PlacePoof {
        timer: Timer::from_seconds(POOF_SECONDS, TimerMode::Once),
        material,
      },
      Name::new("PlacePoof"),
    ));
  }
}

fn update_place_poof(
  mut commands: Commands,
  time: Res<Time>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut poofs: Query<(Entity, &mut PlacePoof, &mut Transform)>,
) {
  for (entity, mut poof, mut transform) in &mut poofs {
    poof.timer.tick(time.delta());
    let t = (poof.timer.elapsed_secs() / POOF_SECONDS).clamp(0., 1.);
    transform.scale = Vec3::splat(1.0 + 2.0 * t);
    if let Some(material) = materials.get_mut(&poof.material) {
      material.color.set_a(0.6 * (1.0 - t));
    }
    if poof.timer.finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn cleanup_place_poofs(mut commands: Commands, poofs: Query<Entity, With<PlacePoof>>) {
  for entity in &poofs {
    commands.entity(entity).despawn_recursive();
  }
}

/// A rising, fading world-space label (damage / income numbers).
const FLOAT_SECONDS: f32 = 0.7;
const FLOAT_RISE_SPEED: f32 = 60.0;

#[derive(Component)]
pub struct FloatingText {
  timer: Timer,
}

fn spawn_floating_text(
  mut commands: Commands,
  assets: Res<GameAssets>,
  mut events: EventReader<FloatingTextEvent>,
) {
  for event in events.iter() {
    commands.spawn((
      Text2dBundle {
        text: Text::from_section(
          event.text.clone(),
          TextStyle {
            font: assets.font.clone(),
            font_size: 26.0,
            color: event.color,
          },
        ),
        // Offset to the top-right so the tall slime jump sprite doesn't cover it.
        transform: Transform::from_translation(event.position + Vec3::new(30., 30., 200.)),
        ..default()
      },
      FloatingText {
        timer: Timer::from_seconds(FLOAT_SECONDS, TimerMode::Once),
      },
      Name::new("FloatingText"),
    ));
  }
}

fn update_floating_text(
  mut commands: Commands,
  time: Res<Time>,
  mut texts: Query<(Entity, &mut FloatingText, &mut Transform, &mut Text)>,
) {
  for (entity, mut float, mut transform, mut text) in &mut texts {
    float.timer.tick(time.delta());
    let t = (float.timer.elapsed_secs() / FLOAT_SECONDS).clamp(0., 1.);
    transform.translation.y += FLOAT_RISE_SPEED * time.delta_seconds();
    text.sections[0].style.color.set_a(1.0 - t);
    if float.timer.finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn cleanup_floating_text(mut commands: Commands, texts: Query<Entity, With<FloatingText>>) {
  for entity in &texts {
    commands.entity(entity).despawn_recursive();
  }
}

/// Camera trauma-based screen shake, triggered by base damage.
const SHAKE_MAX_OFFSET: f32 = 16.0;
const SHAKE_DECAY: f32 = 1.6;
const SHAKE_PER_HIT: f32 = 0.6;

#[derive(Resource, Default)]
pub struct ScreenShake {
  trauma: f32,
  /// Unshaken camera position, captured when a shake begins.
  home: Option<Vec3>,
}

fn screen_shake(
  time: Res<Time>,
  mut shake: ResMut<ScreenShake>,
  mut damage_events: EventReader<BaseDamagedEvent>,
  mut camera: Query<&mut Transform, With<MainCamera>>,
) {
  let Ok(mut transform) = camera.get_single_mut() else {
    return;
  };

  if damage_events.iter().next().is_some() {
    damage_events.clear();
    // Capture the resting position only when not already shaking.
    if shake.home.is_none() {
      shake.home = Some(transform.translation);
    }
    shake.trauma = (shake.trauma + SHAKE_PER_HIT).min(1.0);
  }

  let Some(home) = shake.home else {
    return;
  };

  if shake.trauma > 0.0 {
    // Quadratic falloff feels punchier than linear.
    let amount = shake.trauma * shake.trauma;
    let dx = (rand::random::<f32>() * 2.0 - 1.0) * SHAKE_MAX_OFFSET * amount;
    let dy = (rand::random::<f32>() * 2.0 - 1.0) * SHAKE_MAX_OFFSET * amount;
    transform.translation.x = home.x + dx;
    transform.translation.y = home.y + dy;
    shake.trauma = (shake.trauma - SHAKE_DECAY * time.delta_seconds()).max(0.0);
  } else {
    // Settle back exactly and release the captured home for the next shake.
    transform.translation = home;
    shake.home = None;
  }
}

/// A brief bright spark at each bullet impact. Uses a plain Sprite quad (no
/// per-hit mesh/material assets) since hits are very frequent.
const SPARK_SECONDS: f32 = 0.14;

#[derive(Component)]
pub struct HitSpark {
  timer: Timer,
}

fn spawn_hit_spark(mut commands: Commands, mut hits: EventReader<EnemyHitEvent>) {
  for hit in hits.iter() {
    commands.spawn((
      SpriteBundle {
        sprite: Sprite {
          color: Color::rgba(1.0, 0.95, 0.6, 0.9),
          custom_size: Some(Vec2::splat(14.0)),
          ..default()
        },
        transform: Transform::from_translation(hit.position.truncate().extend(150.)),
        ..default()
      },
      HitSpark {
        timer: Timer::from_seconds(SPARK_SECONDS, TimerMode::Once),
      },
      Name::new("HitSpark"),
    ));
  }
}

fn update_hit_spark(
  mut commands: Commands,
  time: Res<Time>,
  mut sparks: Query<(Entity, &mut HitSpark, &mut Transform, &mut Sprite)>,
) {
  for (entity, mut spark, mut transform, mut sprite) in &mut sparks {
    spark.timer.tick(time.delta());
    let t = (spark.timer.elapsed_secs() / SPARK_SECONDS).clamp(0., 1.);
    transform.scale = Vec3::splat(1.0 + 1.4 * t);
    sprite.color.set_a(0.9 * (1.0 - t));
    if spark.timer.finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn cleanup_hit_sparks(mut commands: Commands, sparks: Query<Entity, With<HitSpark>>) {
  for entity in &sparks {
    commands.entity(entity).despawn_recursive();
  }
}

/// Full-screen red flash that pulses when the base takes damage.
const FLASH_PEAK_ALPHA: f32 = 0.35;
const FLASH_FADE_PER_SEC: f32 = 1.2;

#[derive(Component)]
pub struct DamageFlash;

fn spawn_damage_flash(mut commands: Commands) {
  commands.spawn((
    NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
        position_type: PositionType::Absolute,
        ..default()
      },
      background_color: Color::rgba(0.8, 0.0, 0.0, 0.0).into(),
      // Never intercept clicks (tower placement etc.).
      focus_policy: FocusPolicy::Pass,
      z_index: ZIndex::Global(50),
      ..default()
    },
    DamageFlash,
    Name::new("DamageFlash"),
  ));
}

fn update_damage_flash(
  time: Res<Time>,
  mut damage_events: EventReader<BaseDamagedEvent>,
  mut flash: Query<&mut BackgroundColor, With<DamageFlash>>,
) {
  let Ok(mut color) = flash.get_single_mut() else {
    return;
  };
  if damage_events.iter().next().is_some() {
    damage_events.clear();
    color.0.set_a(FLASH_PEAK_ALPHA);
  } else {
    let new_alpha = (color.0.a() - FLASH_FADE_PER_SEC * time.delta_seconds()).max(0.0);
    color.0.set_a(new_alpha);
  }
}

fn cleanup_damage_flash(mut commands: Commands, flashes: Query<Entity, With<DamageFlash>>) {
  for entity in &flashes {
    commands.entity(entity).despawn_recursive();
  }
}

/// An expanding orange ring marking a splash/AoE hit, sized to the splash radius.
const SPLASH_SECONDS: f32 = 0.35;

#[derive(Component)]
pub struct SplashRing {
  timer: Timer,
  material: Handle<ColorMaterial>,
  base_scale: f32,
}

fn spawn_splash_ring(
  mut commands: Commands,
  mut events: EventReader<SplashEvent>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for splash in events.iter() {
    let material = materials.add(ColorMaterial::from(Color::rgba(1.0, 0.5, 0.1, 0.55)));
    commands.spawn((
      MaterialMesh2dBundle {
        // Unit circle; scaled up to the splash radius over its lifetime.
        mesh: meshes.add(shape::Circle::new(1.0).into()).into(),
        material: material.clone(),
        transform: Transform::from_translation(splash.position.truncate().extend(140.)),
        ..default()
      },
      SplashRing {
        timer: Timer::from_seconds(SPLASH_SECONDS, TimerMode::Once),
        material,
        base_scale: splash.radius,
      },
      Name::new("SplashRing"),
    ));
  }
}

fn update_splash_ring(
  mut commands: Commands,
  time: Res<Time>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut rings: Query<(Entity, &mut SplashRing, &mut Transform)>,
) {
  for (entity, mut ring, mut transform) in &mut rings {
    ring.timer.tick(time.delta());
    let t = (ring.timer.elapsed_secs() / SPLASH_SECONDS).clamp(0., 1.);
    // Grow from ~half to full splash radius, fading out.
    transform.scale = Vec3::splat(ring.base_scale * (0.5 + 0.5 * t));
    if let Some(material) = materials.get_mut(&ring.material) {
      material.color.set_a(0.55 * (1.0 - t));
    }
    if ring.timer.finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn cleanup_splash_rings(mut commands: Commands, rings: Query<Entity, With<SplashRing>>) {
  for entity in &rings {
    commands.entity(entity).despawn_recursive();
  }
}

/// Tint enemies by status: knockback = purple, poison = green, slow/stun = blue.
/// Skips enemies mid hit-flash (the flash owns their colour briefly).
fn tint_slowed(
  mut enemies: Query<
    (
      &mut TextureAtlasSprite,
      Option<&Invisible>,
      Option<&Slowed>,
      Option<&Poisoned>,
      Option<&KnockedBack>,
    ),
    (With<Enemy>, Without<HitFlash>),
  >,
) {
  for (mut sprite, invisible, slowed, poisoned, knocked) in &mut enemies {
    let mut color = if knocked.is_some() {
      Color::rgb(0.85, 0.4, 1.0)
    } else if poisoned.is_some() {
      Color::rgb(0.4, 1.0, 0.4)
    } else if slowed.is_some() {
      Color::rgb(0.45, 0.6, 1.0)
    } else {
      Color::WHITE
    };
    // Keep invisible enemies ghostly even while a status tints them.
    if invisible.is_some() {
      color.set_a(INVISIBLE_ALPHA);
    }
    sprite.color = color;
  }
}

/// A short-lived lightning segment drawn between two chain targets.
const BOLT_SECONDS: f32 = 0.16;

pub struct ChainBoltEvent {
  pub from: Vec3,
  pub to: Vec3,
}

#[derive(Component)]
pub struct ChainBolt {
  timer: Timer,
}

fn spawn_chain_bolt(mut commands: Commands, mut events: EventReader<ChainBoltEvent>) {
  for bolt in events.iter() {
    let from = bolt.from.truncate();
    let to = bolt.to.truncate();
    let delta = to - from;
    let length = delta.length().max(1.0);
    let midpoint = (from + to) / 2.0;
    let angle = delta.y.atan2(delta.x);
    commands.spawn((
      SpriteBundle {
        sprite: Sprite {
          color: Color::rgba(1.0, 1.0, 0.3, 0.9),
          custom_size: Some(Vec2::new(length, 3.0)),
          ..default()
        },
        transform: Transform::from_translation(midpoint.extend(160.))
          .with_rotation(Quat::from_rotation_z(angle)),
        ..default()
      },
      ChainBolt {
        timer: Timer::from_seconds(BOLT_SECONDS, TimerMode::Once),
      },
      Name::new("ChainBolt"),
    ));
  }
}

fn update_chain_bolt(
  mut commands: Commands,
  time: Res<Time>,
  mut bolts: Query<(Entity, &mut ChainBolt, &mut Sprite)>,
) {
  for (entity, mut bolt, mut sprite) in &mut bolts {
    bolt.timer.tick(time.delta());
    let t = (bolt.timer.elapsed_secs() / BOLT_SECONDS).clamp(0., 1.);
    sprite.color.set_a(0.9 * (1.0 - t));
    if bolt.timer.finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn cleanup_chain_bolts(mut commands: Commands, bolts: Query<Entity, With<ChainBolt>>) {
  for entity in &bolts {
    commands.entity(entity).despawn_recursive();
  }
}

fn lerp_color(from: Color, to: Color, t: f32) -> Color {
  Color::rgb(
    from.r() + (to.r() - from.r()) * t,
    from.g() + (to.g() - from.g()) * t,
    from.b() + (to.b() - from.b()) * t,
  )
}
