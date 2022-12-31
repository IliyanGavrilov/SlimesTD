use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

mod target;
use crate::target::*;

// Debugging
use bevy_editor_pls::*;
//use crate::target::Target;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Base {
  health: i32
}

//#[derive(Component)] // Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
  bullet_spawn_offset: Vec3,
  damage: i32,
  attack_speed: Timer,
  range: i32,
  price: i32,
  sell_price: i32,
  target: TargettingPriority
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
  direction: Vec3,
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
    .add_system(move_enemies)
    .add_system(move_bullets)
    .add_system(despawn_bullets)
    
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
    .register_type::<Base>()
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
      target: TargettingPriority::CLOSE
      //..default()
    })
    .insert(Name::new("Tower"));
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

fn move_enemies(mut enemies: Query<(&Enemy, &mut Transform)>, time: Res<Time>) {
  for (enemy, mut transform) in &mut enemies {
    transform.translation.y += enemy.speed * time.delta_seconds();
  }
}

fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
  for (bullet, mut transform) in &mut bullets {
    transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
  }
}

fn tower_shooting(
  mut commands: Commands,
  assets: Res<GameAssets>, // Bullet assets
  mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
  enemies: Query<&GlobalTransform, With<Enemy>>, // Gets all entities With the Enemy component
  time: Res<Time>,
) {
  for (tower_entity, mut tower, transform) in &mut towers {
    tower.attack_speed.tick(time.delta());
  
    let bullet_spawn_pos = transform.translation() + tower.bullet_spawn_offset;
    
    // If the attack cooldown finished, spawn bullet
    if tower.attack_speed.finished() {
      let direction = match &tower.target {
        TargettingPriority::FIRST => first_enemy_direction(&enemies, bullet_spawn_pos),
        TargettingPriority::LAST => last_enemy_direction(&enemies, bullet_spawn_pos),
        TargettingPriority::CLOSE => closest_enemy_direction(&enemies, bullet_spawn_pos),
        TargettingPriority::STRONGEST => strongest_enemy_direction(&enemies, bullet_spawn_pos),
        TargettingPriority::WEAKEST => weakest_enemy_direction(&enemies, bullet_spawn_pos)
      };
      
      // If there is an enemy in the tower's range!!! (if direction != None), then shoot bullet
      if let Some(direction) = direction {
        // Make bullet a child of tower
        commands.entity(tower_entity).with_children(|commands| {
          commands.spawn(SpriteBundle {
            texture: assets.bullet.clone(),
            transform: Transform::from_translation(tower.bullet_spawn_offset),
            sprite: Sprite {
              flip_x: true,
              custom_size: Some(Vec2::new(25., 25.)),
                ..default()
            },
            ..default()
          })
            .insert(Bullet {
              direction,
              speed: 500.,
              lifetime: Timer::from_seconds(100., TimerMode::Once)
            })
            .insert(Name::new("Bullet"));
        });
      }
    }
  }
}

fn despawn_bullets(
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