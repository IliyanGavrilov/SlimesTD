use bevy::prelude::*;
use crate::{GameAssets, GameState, MainCamera, Player, spawn_tower, TowerType};
use strum::IntoEnumIterator;
use bevy::sprite::MaterialMesh2dBundle;

pub struct TowerButtonPlugin;

impl Plugin for TowerButtonPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(GameState::Gameplay)
        .with_system(generate_ui))
      .add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(tower_button_interaction)
        .with_system(place_tower)
        .with_system(lock_tower_buttons.after(generate_ui)));
  }
}

// Marker component to despawn buttons in UI
#[derive(Component)]
pub struct TowerUIRoot;

#[derive(Component)]
pub struct SpriteFollower;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TowerButtonState {
  price: u32
}

fn lock_tower_buttons(
  mut buttons: Query<(&mut TowerButtonState, &TowerType)>,
  mut button_images: Query<(&mut UiImage, &TowerType)>,
  player: Query<&Player>,
  assets: Res<GameAssets>
) {
  let player = player.single();
  
  for (state, tower_type) in &mut buttons {
    for (mut image, button_tower_type) in button_images.iter_mut() {
      if player.money >= state.price as usize {
        if button_tower_type == tower_type {
          image.0 = assets.get_button_asset(*tower_type);
        }
      }
      else {
        if button_tower_type == tower_type {
          image.0 = assets.get_button_locked_asset(*tower_type);
        }
      }
    }
  }
}

// Convert cursor position from window/screen position to world position
pub fn window_to_world_pos(
  window: &Window,
  cursor_pos: Vec2,
  camera: &Camera,
  camera_transform: &GlobalTransform
) -> Vec3 {
  // get the size of the window
  let window_size = Vec2::new(window.width() as f32, window.height() as f32);
  
  // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
  // Normalized device coordinates
  let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
  
  // matrix for undoing the projection and camera transform
  let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
  
  // use it to convert ndc to world-space coordinates
  let mut world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
  
  world_pos.z = 0.;
  
  return world_pos;
}

fn place_tower(
  mut commands: Commands,
  mut query: Query<(Entity, &mut Transform, &TowerType), With<SpriteFollower>>,
  assets: Res<GameAssets>,
  mouse: Res<Input<MouseButton>>,
  windows: Res<Windows>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut player: Query<&mut Player>
) {
  let window = windows.get_primary().unwrap();
  let (camera, camera_transform) = camera_query.single();
  let mut player = player.single_mut();
  
  for (entity, mut transform, tower_type) in query.iter_mut() {
    // Sprite follows mouse until tower is placed or discarded
    if let Some(position) = window.cursor_position() {
      transform.translation =
        window_to_world_pos(window, position, camera, camera_transform);
    }
    // Spawn the tower if user clicks with mouse button in a valid tower placement zone!!!
    if mouse.just_pressed(MouseButton::Left) {
      if let Some(screen_pos) = window.cursor_position() {
        player.money -= tower_type.get_price() as usize;
        commands.entity(entity).despawn_recursive();
        spawn_tower(&mut commands,
                    *tower_type,
                    &assets,
                    window_to_world_pos(window,
                                        screen_pos,
                                        camera,
                                        camera_transform));
      }
    } // Discard tower
    else if mouse.just_pressed(MouseButton::Right) || window.cursor_position().is_none() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn spawn_sprite_follower(
  commands: &mut Commands,
  window: &Window,
  camera: &Camera,
  camera_transform: &GlobalTransform,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<ColorMaterial>>,
  tower_type: &TowerType,
  assets: &Res<GameAssets>
) {
  // Spawn component that alerts the place_tower() system that a button has been pressed
  // and it starts moving a sprite with the cursor until the tower is placed
  if let Some(position) = window.cursor_position() {
    commands.spawn(SpriteBundle {
      texture: assets.get_tower_asset(*tower_type),
      transform: Transform::from_translation(
        window_to_world_pos(window, position, camera, camera_transform)),
      ..default()
    })
      .insert(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(tower_type.get_range() as f32).into())
          .into(),
        material: materials.add(ColorMaterial::from(
          Color::rgba_u8(0, 0, 0, 85))),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
      })
      .insert(SpriteFollower)
      .insert(*tower_type);
  }
}

fn tower_button_interaction(
  mut commands: Commands,
  assets: Res<GameAssets>,
  interaction: Query<(&Interaction, &TowerType, &TowerButtonState), (Changed<Interaction>, With<Button>)>,
  mut images: Query<(&mut UiImage, &TowerType)>,
  windows: Res<Windows>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  keys: Res<Input<KeyCode>>,
  query: Query<&SpriteFollower>,
  player: Query<& Player>
) {
  let window = windows.get_primary().unwrap();
  let (camera, camera_transform) = camera_query.single();
  let player = player.single();
  
  // If player can afford to buy tower
  
  // Keyboard shortcuts
  if query.is_empty() { // Spawn one tower at a time
    if keys.just_pressed(KeyCode::Key1) &&
      player.money >= TowerType::Nature.get_price() as usize {
      info!("Spawning: Nature wizard");
      
      spawn_sprite_follower(&mut commands, window, camera, camera_transform,
                            &mut meshes, &mut materials, &TowerType::Nature, &assets);
    }
    else if keys.just_pressed(KeyCode::Key2) &&
      player.money >= TowerType::Fire.get_price() as usize {
      info!("Spawning: Fire wizard");
  
      spawn_sprite_follower(&mut commands, window, camera, camera_transform,
                            &mut meshes, &mut materials, &TowerType::Fire, &assets);
    }
    else if keys.just_pressed(KeyCode::Key3) &&
      player.money >= TowerType::Ice.get_price() as usize {
      info!("Spawning: Ice wizard");
  
      spawn_sprite_follower(&mut commands, window, camera, camera_transform,
                            &mut meshes, &mut materials, &TowerType::Ice, &assets);
    }
    else if keys.just_pressed(KeyCode::Key4) &&
      player.money >= TowerType::Dark.get_price() as usize{
      info!("Spawning: Dark wizard");
  
      spawn_sprite_follower(&mut commands, window, camera, camera_transform,
                            &mut meshes, &mut materials, &TowerType::Dark, &assets);
    }
    else if keys.just_pressed(KeyCode::Key5) &&
      player.money >= TowerType::Mage.get_price() as usize {
      info!("Spawning: Mage wizard");
  
      spawn_sprite_follower(&mut commands, window, camera, camera_transform,
                            &mut meshes, &mut materials, &TowerType::Mage, &assets);
    }
    else if keys.just_pressed(KeyCode::Key6) &&
      player.money >= TowerType::Archmage.get_price() as usize {
      info!("Spawning: Archmage wizard");
  
      spawn_sprite_follower(&mut commands, window, camera, camera_transform,
                            &mut meshes, &mut materials, &TowerType::Archmage, &assets);
    }
  }
  
  for (interaction, tower_type, state) in &interaction {
    if player.money >= state.price as usize {
      match interaction {
        Interaction::Clicked => {
          info!("Spawning: {tower_type} wizard");
          if query.is_empty() { // Spawn one tower at a time
            // Change button UI
            for (mut image, button_tower_type) in images.iter_mut() {
              if button_tower_type == tower_type {
                image.0 = assets.get_button_pressed_asset(*tower_type);
              }
            }
            
            // Spawn tower sprite following mouse
            spawn_sprite_follower(&mut commands, window, camera, camera_transform,
                                  &mut meshes, &mut materials, tower_type, &assets);
          }
        }
        Interaction::Hovered => {
          // Change button UI
          for (mut image, button_tower_type) in images.iter_mut() {
            if button_tower_type == tower_type {
              image.0 = assets.get_button_hovered_asset(*tower_type);
            }
          }
        }
        Interaction::None => {
          // Change button UI
          for (mut image, button_tower_type) in images.iter_mut() {
            if button_tower_type == tower_type {
              image.0 = assets.get_button_asset(*tower_type);
            }
          }
        }
      }
    }
  }
}

// Creating a UI menu on the whole screen with buttons
fn generate_ui(mut commands: Commands, assets: Res<GameAssets>) {
  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
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
              size: Size::new(Val::Px(80.), Val::Px(80.)),
              align_self: AlignSelf::FlexEnd, // Bottom of screen
              margin: UiRect::all(Val::Percent(2.)),
              ..default()
            },
            image: assets.get_button_asset(i).into(),
            ..default()
          })
          .insert(TowerButtonState {
            price: i.get_price()
          })
          .insert(i)
          .insert(Name::new("TowerButton"));
      }
    })
    .insert(Name::new("TowerButtons"));
}