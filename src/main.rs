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

// Creating a UI menu on the whole screen
// fn generate_ui(mut commands: Commands) {
//   commands
//     .spawn(NodeBundle {
//       style: Style {
//         size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//         justify_content: JustifyContent::Center,
//         ..default()
//       },
//       ..default()
//     })
//     .insert(TowerUIRoot) // Marker component
//     .with_children(|commands| { // Make the buttons children of the menu
//       for i in 0..3 {
//         commands.spawn(ButtonBundle {
//           style: Style {
//
//           }
//         })
//       }
//     });
// }

fn main() {
  App::new()
    // Background of window. Set colour of screen on each refresh
    .insert_resource(ClearColor(CLEAR))
    
    .add_startup_system(spawn_basic_scene)
    .add_startup_system(spawn_camera)
    // Load assets before the startup stage, so we can use them in spawn_basic_scene()
    .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
    .add_system(animate_enemy_sprite)
    
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
        title: "Wizards vs Slimes Tower Defence".to_string(),
        ..default()
      },
      ..default()
    }))
    
    //.add_system(generate_ui)
    
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
  pub enemy: Handle<TextureAtlas>,
  pub bullet: Handle<Image>
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct AnimationIndices {
  first: usize,
  last: usize
}

fn load_assets(
  mut commands: Commands,
  assets_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
  commands.insert_resource(GameAssets {
    tower: assets_server.load("wizard_fire.png"),
    enemy: texture_atlases.add(
      TextureAtlas::from_grid(assets_server.load("slime_jump.png"),
                             Vec2::new(50., 90.),
                             10, 8,
                             Some(Vec2::new(5., 5.)), None)),
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