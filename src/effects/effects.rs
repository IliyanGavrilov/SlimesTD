use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::{game_not_paused, Enemy, EnemyDeathEvent, GameAssets, GameState, MainCamera, Tower};

/// Visual juice: small, asset-free feedback effects (enemy hit flash, …).
pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<EnemyHitEvent>()
      .add_event::<FloatingTextEvent>()
      .add_event::<BaseDamagedEvent>()
      .init_resource::<ScreenShake>()
      .add_systems(
        (
          apply_hit_flash.run_if(game_not_paused),
          update_hit_flash.run_if(game_not_paused),
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
      .add_system(cleanup_death_pops.in_schedule(OnExit(GameState::Gameplay)))
      .add_system(cleanup_place_poofs.in_schedule(OnExit(GameState::Gameplay)))
      .add_system(cleanup_floating_text.in_schedule(OnExit(GameState::Gameplay)));
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
  mut pops: Query<(Entity, &mut DeathPop, &mut Transform, &mut TextureAtlasSprite)>,
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
        // Above sprites; small offset so it starts just over the source.
        transform: Transform::from_translation(event.position + Vec3::new(0., 18., 200.)),
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
  let Ok(mut transform) = camera.get_single_mut() else { return; };

  if damage_events.iter().next().is_some() {
    damage_events.clear();
    // Capture the resting position only when not already shaking.
    if shake.home.is_none() {
      shake.home = Some(transform.translation);
    }
    shake.trauma = (shake.trauma + SHAKE_PER_HIT).min(1.0);
  }

  let Some(home) = shake.home else { return; };

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

fn lerp_color(from: Color, to: Color, t: f32) -> Color {
  Color::rgb(
    from.r() + (to.r() - from.r()) * t,
    from.g() + (to.g() - from.g()) * t,
    from.b() + (to.b() - from.b()) * t,
  )
}
