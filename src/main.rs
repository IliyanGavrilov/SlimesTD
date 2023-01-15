use bevy::prelude::*;
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

fn main() {
  App::new()
    // Background of window. Set colour of screen on each refresh
    .insert_resource(ClearColor(CLEAR))
    
    .add_startup_system(spawn_basic_scene)
    .add_startup_system(spawn_camera)
    // Load assets before the startup stage, so we can use them in spawn_basic_scene()
    .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
    
    .add_plugin(TowerPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(BulletPlugin)
    .add_plugin(MovementPlugin)
    
    // Add basic game functionality - window, game tick, renderer,
    // asset loading, UI system, input, startup systems, etc.
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      window: WindowDescriptor {
        title: "Tower Defence".to_string(),
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

#[derive(Resource)]
pub struct GameAssets {
  pub tower: Handle<Image>,
  pub enemy: Handle<Image>,
  pub bullet: Handle<Image>
}

fn load_assets(mut commands: Commands, assets_server: Res<AssetServer>) {
  commands.insert_resource(GameAssets {
    tower: assets_server.load("tower.png"),
    enemy: assets_server.load("enemy.png"),
    bullet: assets_server.load("fireball.png")
  })
}

fn spawn_basic_scene(
  mut commands: Commands,
  assets: Res<GameAssets> // Tower and enemy assets
) {
  // Enemy
  commands.spawn(SpriteBundle {
    texture: assets.enemy.clone(),
    transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
    sprite: Sprite {
      custom_size: Some(Vec2::new(50., 50.)),
      ..default()
    },
    ..default()
  })
    .insert(Enemy {
      health: 5,
    })
    .insert(Movement {
      direction: Vec3::new(-200., 999999., 0.),
      speed: 5.
    })
    .insert(Name::new("Enemy")); // !!! Debug
  
  // Enemy 2
  commands.spawn(SpriteBundle {
    texture: assets.enemy.clone(),
    transform: Transform::from_translation(Vec3::new(-200., -100., 0.)),
    sprite: Sprite {
      custom_size: Some(Vec2::new(50., 50.)),
      ..default()
    },
    ..default()
  })
    .insert(Enemy {
      health: 5,
    })
    .insert(Movement {
      direction: Vec3::new(-200., 999999., 0.),
      speed: 5.
    })
    .insert(Name::new("Enemy 2")); // !!! Debug
  
  // Tower
  commands.spawn(SpriteBundle {
    texture: assets.tower.clone(),
    transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
    sprite: Sprite {
      custom_size: Some(Vec2::new(50., 50.)),
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