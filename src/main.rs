use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
// Debugging
use bevy_editor_pls::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

//#[derive(Component)] // Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
  shooting_timer: Timer
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
  lifetime: Timer
}

fn main() {
  App::new()
    // Background of window. Set colour of screen on each refresh
    .insert_resource(ClearColor(CLEAR))
    
    .add_startup_system(spawn_basic_scene)
    .add_startup_system(spawn_camera)
    
    .add_system(tower_shooting)
    .add_system(bullet_despawn)
    
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
    .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .register_type::<Tower>()
    .register_type::<Bullet>()
    .run();
}

fn spawn_basic_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  // Circle
  commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(shape::Circle::new(50.).into()).into(),
    material: materials.add(ColorMaterial::from(Color::CYAN)),
    transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
    ..default()
  })
    .insert(Tower {
      shooting_timer: Timer::from_seconds(1., TimerMode::Repeating)
    })
    .insert(Name::new("Circle"));
  
  commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(shape::Circle::new(50.).into()).into(),
    material: materials.add(ColorMaterial::from(Color::RED)),
    transform: Transform::from_translation(Vec3::new(100., 0., 10.)),
    ..default()
  })
    .insert(Name::new("Circle 2"));
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

fn tower_shooting(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut towers: Query<&mut Tower>,
  time: Res<Time>
) {
  for mut tower in &mut towers {
    tower.shooting_timer.tick(time.delta());
    // If the attack cooldown finished, spawn bullet
    if tower.shooting_timer.finished() {
      let spawn_transform =
        Transform::from_translation(Vec3::new(0., 0., 0.));
  
      commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(10.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: spawn_transform,
        ..default()
      })
        .insert(Bullet {
          lifetime: Timer::from_seconds(0.5, TimerMode::Once)
        })
        .insert(Name::new("Bullet"));
    }
  }
}

fn bullet_despawn(
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