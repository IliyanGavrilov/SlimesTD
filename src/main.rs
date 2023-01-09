use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// Debugging
use bevy_editor_pls::*;
//use bevy_inspector_egui::WorldInspectorPlugin;

mod base;
mod tower;
mod enemy;
mod bullet;
mod targeting_priority;

// mod startup_setup;
// mod asset_loading;

pub use base::*;
pub use enemy::*;
pub use tower::*;
pub use bullet::*;

// Background of window. The colour of the screen on each refresh
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
  App::new()
    // Background of window. Set colour of screen on each refresh
    .insert_resource(ClearColor(CLEAR))
    
    .add_startup_system(spawn_basic_scene)
    .add_startup_system(spawn_camera)
    .add_startup_system(load_assets)
    
    .add_plugin(TowerPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(BulletPlugin)
    
    // Add basic game functionality - window, game tick, renderer,
    // asset loading, UI system, input, startup systems, etc.
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      window: WindowDescriptor {
        title: "Tower Defence".to_string(),
        ..default()
      },
      ..default()
    }))
    
    // Debugging
    .add_plugin(EditorPlugin)
    // .add_plugin(WorldInspectorPlugin)
    .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    // .register_type::<Base>()
    // .register_type::<Tower>()
    // .register_type::<Enemy>()
    // .register_type::<Bullet>()
    .run();
}

#[derive(Resource)]
pub struct GameAssets {
  pub bullet: Handle<Image>
}

fn load_assets(mut commands: Commands, assets_server: Res<AssetServer>) {
  commands.insert_resource(GameAssets {
    bullet: assets_server.load("bullet.png"),
  })
}

fn spawn_basic_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  // Enemy
  commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(shape::Circle::new(15.).into()).into(),
    material: materials.add(ColorMaterial::from(Color::RED)),
    transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
    ..default()
  })
    .insert(Enemy {
      health: 5,
      speed: 5.
    })
    .insert(Name::new("Enemy"));
  
  // Enemy 2
  commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(shape::Circle::new(15.).into()).into(),
    material: materials.add(ColorMaterial::from(Color::RED)),
    transform: Transform::from_translation(Vec3::new(-200., -100., 0.)),
    ..default()
  })
    .insert(Enemy {
      health: 5,
      speed: 5.
    })
    .insert(Name::new("Enemy 2"));
  
  // Tower
  commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(shape::Circle::new(25.).into()).into(),
    material: materials.add(ColorMaterial::from(Color::CYAN)),
    transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
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
    .insert(Name::new("Tower"));
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}