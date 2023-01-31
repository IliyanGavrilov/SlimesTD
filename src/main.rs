use bevy::prelude::*;
use bevy::window::PresentMode;
// !!!Debugging
use bevy_editor_pls::*;

mod base;
pub use base::*;
mod tower;
pub use tower::*;
mod enemy;
pub use enemy::*;
mod bullet;
pub use bullet::*;
mod movement;
pub use movement::*;
mod targeting_priority;

// Background of window. The colour of the screen on each refresh
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

use strum::IntoEnumIterator;

// Creating a UI menu on the whole screen with buttons
fn generate_ui(mut commands: Commands, assets_server: Res<AssetServer>) {
  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    })
    .insert(TowerUIRoot) // Marker component
    .with_children(|commands| { // Make the buttons children of the menu
      for i in TowerType::iter() {
        commands
          .spawn(ButtonBundle {
            style: Style {
              size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),
              align_self: AlignSelf::FlexEnd, // Bottom of screen
              margin: UiRect::all(Val::Percent(2.0)),
              ..default()
            },
            image: assets_server.load(i.path()).clone().into(),
            ..default()
          })
          .insert(i);
      }
    });
}

fn load(time: Res<Time>) {
  Timer::from_seconds(5., TimerMode::Once).tick(time.delta());
}

fn main() {
  App::new()
    // Background of window. Set colour of screen on each refresh
    .insert_resource(ClearColor(CLEAR))
  
    .add_startup_system_to_stage(StartupStage::PreStartup, load)
    
    .add_startup_system(spawn_basic_scene)
    .add_startup_system(spawn_camera)
    // Load assets before the startup stage, so we can use them in spawn_basic_scene()
    .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
    .add_system(animate_enemy_sprite)
    
    .add_startup_system(generate_ui)
    
    .add_plugin(TowerPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(BulletPlugin)
    .add_plugin(MovementPlugin)
    
    // Add basic game functionality - window, game tick, renderer,
    // asset loading, UI system, input, startup systems, etc.
    .add_plugins(DefaultPlugins
      .set(ImagePlugin::default_nearest()) // Prevent blurry sprites
      .set(WindowPlugin {
      window: WindowDescriptor {
        title: "Slimes Tower Defence".to_string(),
        present_mode: PresentMode::AutoVsync,
        ..default()
      },
      ..default()
    }))
    
    // !!!Debugging
    .add_plugin(EditorPlugin) // Similar to WorldInspectorPlugin
    .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .run();
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct AnimationIndices {
  first: usize,
  last: usize
}

#[derive(Resource)]
pub struct GameAssets {
  // Towers
  pub wizard_nature: Handle<Image>,
  pub wizard_fire: Handle<Image>,
  pub wizard_ice: Handle<Image>,
  pub wizard_dark: Handle<Image>,
  pub wizard_mage: Handle<Image>,
  pub wizard_archmage: Handle<Image>,
  pub tower: Handle<Image>,
  // Tower buttons:
  pub wizard_nature_button: Handle<Image>,
  pub wizard_fire_button: Handle<Image>,
  pub wizard_ice_button: Handle<Image>,
  pub wizard_dark_button: Handle<Image>,
  pub wizard_mage_button: Handle<Image>,
  pub wizard_archmage_button: Handle<Image>,
  // Enemies
  pub enemy: Handle<TextureAtlas>, // !!!! Rename
  // Tower bullets
  pub wizard_nature_bullet: Handle<Image>,
  pub wizard_fire_bullet: Handle<Image>,
  pub wizard_ice_bullet: Handle<Image>,
  pub wizard_dark_bullet: Handle<Image>,
  pub wizard_mage_bullet: Handle<Image>,
  pub wizard_archmage_bullet: Handle<Image>,
  pub bullet: Handle<Image>
}

fn load_assets(
  mut commands: Commands,
  assets_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
  commands.insert_resource(GameAssets {
    // Towers
    wizard_nature: assets_server.load("towers/wizard_nature.png"),
    wizard_fire: assets_server.load("towers/wizard_fire.png"),
    wizard_ice: assets_server.load("towers/wizard_ice.png"),
    wizard_dark: assets_server.load("towers/wizard_dark.png"),
    wizard_mage: assets_server.load("towers/wizard_mage.png"),
    wizard_archmage: assets_server.load("towers/wizard_archmage.png"),
    tower: assets_server.load("towers/wizard_fire.png"),
    // Tower buttons
    wizard_nature_button: assets_server.load("tower_buttons/wizard_nature_button.png"),
    wizard_fire_button: assets_server.load("tower_buttons/wizard_fire_button.png"),
    wizard_ice_button: assets_server.load("tower_buttons/wizard_ice_button.png"),
    wizard_dark_button: assets_server.load("tower_buttons/wizard_dark_button.png"),
    wizard_mage_button: assets_server.load("tower_buttons/wizard_mage_button.png"),
    wizard_archmage_button: assets_server.load("tower_buttons/wizard_archmage_button.png"),
    // Enemies
    enemy: texture_atlases.add(
      TextureAtlas::from_grid(assets_server.load("slime_jump.png"),
                             Vec2::new(50., 90.),
                             10, 8,
                             Some(Vec2::new(5., 5.)), None)),
    // Bullets !!!!!
    wizard_nature_bullet: assets_server.load("tower_bullets/wizard_nature_bullet.png"),
    wizard_fire_bullet: assets_server.load("tower_bullets/wizard_fire_bullet.png"),
    wizard_ice_bullet: assets_server.load("tower_bullets/wizard_ice_bullet.png"),
    wizard_dark_bullet: assets_server.load("tower_bullets/wizard_dark_bullet.png"),
    wizard_mage_bullet: assets_server.load("tower_bullets/wizard_mage_bullet.png"),
    wizard_archmage_bullet: assets_server.load("tower_bullets/wizard_archmage_bullet.png"),
    bullet: assets_server.load("fireball.png")
  })
}

fn animate_enemy_sprite(
  time: Res<Time>,
  mut query: Query<(
    &AnimationIndices,
    &mut AnimationTimer,
    &mut TextureAtlasSprite,
    &GlobalTransform,
    &Movement
  )>
) {
  for (indices,
      mut timer,
      mut sprite,
      transform,
      movement) in &mut query {
    // Change direction based on where enemy is heading
    if transform.translation().x > movement.direction.x {
      sprite.flip_x = true;
    }
    else {
      sprite.flip_x = false;
    }
    
    // Animate sprite
    timer.tick(time.delta());
    
    if timer.just_finished() {
      if sprite.index == indices.last {
        sprite.index = indices.first;
      }
      else {
        sprite.index += 1;
      }
    }
  }
}

fn spawn_basic_scene(
  mut commands: Commands,
  assets: Res<GameAssets> // Tower and enemy assets
) {
  // Enemy
  commands.spawn(SpriteSheetBundle {
    texture_atlas: assets.enemy.clone(),
    transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
    ..default()
  })
    .insert(AnimationIndices {first: 0, last: 9})
    // Animate slime jumping
    .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
    .insert(Enemy {
      health: 5,
    })
    .insert(Movement {
      direction: Vec3::new(-200., 9999999., 0.),
      speed: 15.
    })
    .insert(Name::new("Enemy")); // !!! Debug
  
  // Enemy 2
  commands.spawn(SpriteSheetBundle {
    texture_atlas: assets.enemy.clone(),
    transform: Transform::from_translation(Vec3::new(-200., -100., 0.)),
    ..default()
  })
    .insert(AnimationIndices {first: 10, last: 19})
    // Animate slime jumping
    .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
    .insert(Enemy {
      health: 5,
    })
    .insert(Movement {
      direction: Vec3::new(-200., 999999., 0.),
      speed: 15.
    })
    .insert(Name::new("Enemy 2")); // !!! Debug
  
  // Tower
  commands.spawn(SpriteBundle {
    texture: assets.tower.clone(),
    transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
    sprite: Sprite {
      ..default()
    },
    ..default()
  })
    .insert(Tower {
      bullet_spawn_offset: Vec3::new(15., 0., 0.),
      damage: 1,
      attack_speed: Timer::from_seconds(1., TimerMode::Repeating),
      range: 10,
      price: 100,
      sell_price: (100/3) as i32,
      target: TargetingPriority::CLOSE
      //..default()
    })
    .insert(Name::new("Tower")); // !!! Debug
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}