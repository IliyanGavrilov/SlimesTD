use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::utils::FloatOrd;

mod target;
use target::Target::*;

// Debugging
use bevy_editor_pls::*;
use crate::target::Target;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Resource)]
pub struct Base {
  health: i32
}

//#[derive(Component)] // Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
  position: (f32, f32),
  damage: i32,
  attack_speed: Timer,
  range: i32,
  price: i32,
  sell_price: i32,
  target: Target
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy {
  health: i32,
  speed: f32
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
  direction: Vec2,
  speed: f32,
  lifetime: Timer // temp
}

#[derive(Resource)]
pub struct GameAssets {
  bullet: Handle<Image>
}

fn load_assets(mut commands: Commands, assets_server: Res<AssetServer>) {
  commands.insert_resource(GameAssets {
    bullet: assets_server.load("bullet.png"),
  })
}

fn main() {
  App::new()
    // Background of window. Set colour of screen on each refresh
    .insert_resource(ClearColor(CLEAR))
    
    .add_startup_system(spawn_basic_scene)
    .add_startup_system(spawn_camera)
    .add_startup_system(load_assets)
    
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
    .register_type::<Enemy>()
    .register_type::<Bullet>()
    .run();
}

fn spawn_basic_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.insert_resource(Base {
    health: 100
  });
  
  // Enemy
  commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(shape::Circle::new(50.).into()).into(),
    material: materials.add(ColorMaterial::from(Color::CYAN)),
    transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
    ..default()
  })
    .insert(Enemy {
      health: 5,
      speed: 0.5
    })
    .insert(Name::new("Enemy"));
  
  // Tower
  commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(shape::Circle::new(50.).into()).into(),
    material: materials.add(ColorMaterial::from(Color::RED)),
    transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
    ..default()
  })
    .insert(Tower {
      position: (100., 0.),
      damage: 1,
      attack_speed: Timer::from_seconds(1., TimerMode::Repeating),
      range: 10,
      price: 100,
      sell_price: (100/3) as i32,
      target: FIRST
    })
    .insert(Name::new("Tower"));
}

// fn move_enemy(mut enemies: Query<&Enemy, &mut Transform>, time: Res<Time>) {
//    for (target, mut transform) in &mut enemies {
//      transform.translation.y += target.speed * time.delta_seconds();
//    }
// }

fn spawn_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

// fn tower_shooting(
//   mut commands: Commands,
//   assets: Res<GameAssets>, // Bullet assets
//   mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
//   enemies: Query<&GlobalTransform, With<Enemy>>, // Gets all entities with the Enemy component
//   time: Res<Time>
// ) {
//   for (tower_ent, mut tower, transform) in &mut towers {
//     tower.attack_speed.tick(time.delta());
//
//     // If the attack cooldown finished, spawn bullet
//     if tower.attack_speed.just_finished() {
//       let bullet_spawn_pos = transform.translation(); //+ bullet_offset;
//
//     }
// }


fn tower_shooting(
  mut commands: Commands,
  assets: Res<GameAssets>, // Bullet assets
  mut towers: Query<&mut Tower>,
  enemies: Query<&GlobalTransform, With<Enemy>>, // Gets all entities with the Enemy component
  time: Res<Time>
) {
  for mut tower in &mut towers {
    tower.attack_speed.tick(time.delta());
    // If the attack cooldown finished, spawn bullet
    if tower.attack_speed.finished() {
      let bullet_spawn_pos =
        Transform::from_translation(Vec3::new(0., 0., 0.));

      commands.spawn(SpriteBundle {
        texture: assets.bullet.clone(),
        transform: bullet_spawn_pos,
        sprite: Sprite {
          flip_x: true,
          custom_size: Some(Vec2::new(25., 25.)),
            ..default()
        },
        ..default()
      })
        .insert(Bullet {
          direction: Vec2::new(25., 25.),
          speed: 2.5,
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