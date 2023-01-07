use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

mod tower;
mod enemy;
mod bullet;

mod targeting_priority;
pub use targeting_priority::*;

// Debugging
use bevy_editor_pls::*;

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
  target: TargetingPriority
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
  damage: i32,
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
    .add_system(enemy_killed)
    .add_system(bullet_enemy_collision)
    
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
      target: TargetingPriority::CLOSE
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
        TargetingPriority::FIRST => first_enemy_direction(&enemies, bullet_spawn_pos),
        TargetingPriority::LAST => last_enemy_direction(&enemies, bullet_spawn_pos),
        TargetingPriority::CLOSE => closest_enemy_direction(&enemies, bullet_spawn_pos),
        TargetingPriority::STRONGEST => strongest_enemy_direction(&enemies, bullet_spawn_pos),
        TargetingPriority::WEAKEST => weakest_enemy_direction(&enemies, bullet_spawn_pos)
      };
      
      // If there is an enemy in the tower's range!!! (if direction != None), then shoot bullet
      if let Some(direction) = direction {
        // Rotate bullet, based on enemy location
        let mut angle = direction.angle_between(tower.bullet_spawn_offset);
        if transform.translation().y > direction.y { // flip angle if enemy is below tower
          angle = -angle;
        }
        let bullet_transform = Transform::from_translation(tower.bullet_spawn_offset);
  
        // Make bullet a child of tower
        commands.entity(tower_entity).with_children(|commands| {
          commands.spawn(SpriteBundle {
            texture: assets.bullet.clone(),
            transform: bullet_transform.with_rotation(Quat::from_rotation_z(angle)),
            sprite: Sprite {
              custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            ..default()
          })
            .insert(Bullet {
              direction,
              speed: 750.,
              damage: tower.damage,
              lifetime: Timer::from_seconds(5., TimerMode::Once)
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

fn enemy_killed(mut commands: Commands, enemies: Query<(Entity, &mut Enemy)>) {
  for (entity, enemy) in &enemies {
    if enemy.health <= 0 {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn bullet_enemy_collision(
  mut commands: Commands,
  bullets: Query<(Entity, &Bullet, &GlobalTransform)>,
  mut enemies: Query<(&mut Enemy, &Transform)>
) {
  for (bullet_entity, bullet, bullet_transform) in &bullets {
    for (mut enemy, enemy_transform) in &mut enemies {
      if Vec3::distance(bullet_transform.translation(), enemy_transform.translation) < 20. {
        commands.entity(bullet_entity).despawn_recursive();
        enemy.health -= bullet.damage;
        break;
      }
    }
  }
}