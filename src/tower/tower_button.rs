use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::assets::*;
use crate::tower::*;
use crate::{GameplayUIRoot, GameState, MainCamera, Player};

pub struct TowerButtonPlugin;

impl Plugin for TowerButtonPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(load_tower_type_stats)
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
  
  world_pos.z = 0.5;
  
  return world_pos;
}

#[derive(Resource)]
struct CursorExitedUI(bool);

fn cursor_above_tower_ui(
  window: &Window,
  node_query: &Query<(&Node, &GlobalTransform, &Visibility), With<GameplayUIRoot>>
) -> bool {
  if let Some(pointer_position) = window.cursor_position() {
    for (node,
      global_transform,
      &Visibility{is_visible}) in node_query.iter() {
      if is_visible {
        let node_position = global_transform.translation().xy();
        let half_size = 0.5 * Vec2::new(node.size().x, window.height() * 0.20);
        let min = node_position - half_size;
        let max = node_position + half_size;
        if (min.x .. max.x).contains(&pointer_position.x)
          && (min.y .. max.y).contains(&pointer_position.y) {
          return true;
        }
      }
    }
  }
  return false;
}

fn place_tower(
  mut commands: Commands,
  mut query: Query<(Entity, &mut Transform, &TowerType, &mut Handle<ColorMaterial>),
    With<SpriteFollower>>,
  assets: Res<GameAssets>,
  mouse: Res<Input<MouseButton>>,
  keys: Res<Input<KeyCode>>,
  windows: Res<Windows>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut player: Query<&mut Player>,
  towers: Query<&Transform, (With<Tower>, Without<SpriteFollower>)>,
  mut clicked_tower: Query<Entity, With<TowerUpgradeUI>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  tower_stats: Res<TowerTypeStats>,
  node_query: Query<(&Node, &GlobalTransform, &Visibility), With<GameplayUIRoot>>,
  mut cursor_exited_ui: ResMut<CursorExitedUI> // Flag to check initial mouse exit from button UI
) {
  let window = windows.get_primary().unwrap();
  let (camera, camera_transform) = camera_query.single();
  let mut player = player.single_mut();
  
  for (entity,
    mut transform,
    tower_type,
    mut color) in query.iter_mut() {
    
    if !clicked_tower.is_empty() {
      let entity = clicked_tower.single_mut();
      commands.entity(entity).remove::<(Handle<ColorMaterial>, TowerUpgradeUI)>();
    }
    // Sprite follows mouse until tower is placed or discarded
    if let Some(position) = window.cursor_position() {
      if !cursor_above_tower_ui(&window, &node_query) {
        cursor_exited_ui.0 = true;
      }
      
      transform.translation =
        window_to_world_pos(window, position, camera, camera_transform);
      
        // Tower range becomes red when trying to place on path/invalid tile
        let mouse_on_placed_tower = towers
          .iter()
          .filter(|tower_transform| {
            Vec3::distance(transform.translation, tower_transform.translation) <= 50. })
          .last();
  
        if mouse_on_placed_tower.is_some() {
          *color = materials.add(ColorMaterial::from(
            Color::rgba_u8(202, 0, 0, 150)));
        }
        else {
          *color = materials.add(ColorMaterial::from(
            Color::rgba_u8(0, 0, 0, 85)));
        }
    }
    
    // Spawn the tower if user clicks with mouse button in a valid tower placement zone!!!
    if mouse.just_pressed(MouseButton::Left) &&
      !cursor_above_tower_ui(&window, &node_query) {
      if let Some(screen_pos) = window.cursor_position() {
        cursor_exited_ui.0 = false;
        let mouse_click_pos =
          window_to_world_pos(window, screen_pos, camera, camera_transform);
        
        let mut place_tower = true;
        
        for tower_transform in towers.iter() {
          if Vec3::distance(mouse_click_pos, tower_transform.translation) <= 40. {
            place_tower = false;
          }
        }
        if place_tower {
          player.money -= tower_stats.tower[tower_type].tower.price as usize;
          commands.entity(entity).despawn_recursive();
          spawn_tower(&mut commands,
                      *tower_type,
                      &assets,
                      mouse_click_pos, &mut meshes, &mut materials, &tower_stats);
        }
      }
    } // Discard tower
    else if mouse.just_pressed(MouseButton::Right) || window.cursor_position().is_none() ||
      (cursor_exited_ui.0 && cursor_above_tower_ui(&window, &node_query)) {
      cursor_exited_ui.0 = false;
      commands.entity(entity).despawn_recursive();
    }
    else if keys.just_pressed(KeyCode::Key1) || keys.just_pressed(KeyCode::Key2) ||
      keys.just_pressed(KeyCode::Key3) || keys.just_pressed(KeyCode::Key4) ||
      keys.just_pressed(KeyCode::Key5) || keys.just_pressed(KeyCode::Key6) {
      cursor_exited_ui.0 = false;
      commands.entity(entity).despawn_recursive();
      tower_spawn_from_keyboard_input(&mut commands, &keys, &player, window,
                                      camera, camera_transform, &mut meshes,
                                      &mut materials, &assets, &tower_stats);
    }
  }
}

fn spawn_sprite_follower(
  commands: &mut Commands,
  window: &Window,
  camera: &Camera,
  camera_transform: &GlobalTransform,
  meshes: &mut Assets<Mesh>,
  materials: &mut Assets<ColorMaterial>,
  tower_type: &TowerType,
  assets: &GameAssets,
  tower_stats: &TowerTypeStats
) {
  // Spawn component that alerts the place_tower() system that a button has been pressed
  // and it starts moving a sprite with the cursor until the tower is placed
  if let Some(position) = window.cursor_position() {
    let transform = window_to_world_pos(window, position, camera, camera_transform);
    commands.spawn(SpriteBundle {
      texture: assets.get_tower_asset(*tower_type),
      transform: Transform::from_translation(transform),
      ..default()
    })
      .insert(spawn_tower_range(meshes, materials,
                                tower_stats.tower[&tower_type].tower.range))
      .insert(SpriteFollower)
      .insert(*tower_type);
  }
}

fn tower_button_interaction(
  mut commands: Commands,
  assets: Res<GameAssets>,
  interaction: Query<(&Interaction, &TowerType, &TowerButtonState),
    (Changed<Interaction>, With<Button>)>,
  mut images: Query<(&mut UiImage, &TowerType)>,
  windows: Res<Windows>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  keys: Res<Input<KeyCode>>,
  query: Query<&SpriteFollower>,
  player: Query<& Player>,
  tower_stats: Res<TowerTypeStats>
) {
  let window = windows.get_primary().unwrap();
  let (camera, camera_transform) = camera_query.single();
  let player = player.single();
  
  // Keyboard shortcuts
  if query.is_empty() { // Spawn one tower at a time
    tower_spawn_from_keyboard_input(&mut commands, &keys, &player, window,
                                    camera, camera_transform, &mut meshes,
                                    &mut materials, &assets, &tower_stats);
  }
  
  for (interaction, tower_type, state) in &interaction {
    if player.money >= state.price as usize {
      match interaction {
        Interaction::Clicked => {
          if query.is_empty() { // Spawn one tower at a time
            // Change button UI
            for (mut image, button_tower_type) in images.iter_mut() {
              if button_tower_type == tower_type {
                image.0 = assets.get_button_pressed_asset(*tower_type);
              }
            }
            
            // Spawn tower sprite following mouse
            spawn_sprite_follower(&mut commands, window, camera, camera_transform, &mut meshes,
                                  &mut materials, tower_type, &assets, &tower_stats);
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

fn tower_spawn_from_keyboard_input(
  commands: &mut Commands,
  keys: &Input<KeyCode>,
  player: &Player,
  window: &Window,
  camera: &Camera,
  camera_transform: &GlobalTransform,
  meshes: &mut Assets<Mesh>,
  materials: &mut Assets<ColorMaterial>,
  assets: &GameAssets,
  tower_stats: &TowerTypeStats
) {
  if keys.just_pressed(KeyCode::Key1) &&
    player.money >= tower_stats.tower[&TowerType::Nature].tower.price as usize {
    
    spawn_sprite_follower(commands, window, camera, camera_transform, meshes,
                          materials, &TowerType::Nature, assets, &tower_stats);
  }
  else if keys.just_pressed(KeyCode::Key2) &&
    player.money >= tower_stats.tower[&TowerType::Fire].tower.price as usize {
  
    spawn_sprite_follower(commands, window, camera, camera_transform, meshes,
                          materials, &TowerType::Fire, assets, &tower_stats);
  }
  else if keys.just_pressed(KeyCode::Key3) &&
    player.money >= tower_stats.tower[&TowerType::Ice].tower.price as usize {
  
    spawn_sprite_follower(commands, window, camera, camera_transform, meshes,
                          materials, &TowerType::Ice, assets, &tower_stats);
  }
  else if keys.just_pressed(KeyCode::Key4) &&
    player.money >= tower_stats.tower[&TowerType::Dark].tower.price as usize {
  
    spawn_sprite_follower(commands, window, camera, camera_transform, meshes,
                          materials, &TowerType::Dark, assets, &tower_stats);
  }
  else if keys.just_pressed(KeyCode::Key5) &&
    player.money >= tower_stats.tower[&TowerType::Mage].tower.price as usize {
  
    spawn_sprite_follower(commands, window, camera, camera_transform, meshes,
                          materials, &TowerType::Mage, assets, &tower_stats);
  }
  else if keys.just_pressed(KeyCode::Key6) &&
    player.money >= tower_stats.tower[&TowerType::Archmage].tower.price as usize {
  
    spawn_sprite_follower(commands, window, camera, camera_transform, meshes,
                          materials, &TowerType::Archmage, assets, &tower_stats);
  }
}

// Creating a UI menu on the whole screen with buttons
fn generate_ui(mut commands: Commands, assets: Res<GameAssets>, tower_stats: Res<TowerTypeStats>) {
  commands.insert_resource(CursorExitedUI {
    0: false,
  });
  commands
    .spawn(NodeBundle {
      background_color: BackgroundColor(Color::GOLD),
      style: Style {
        size: Size::new(Val::Percent(100.), Val::Percent(12.)),
        justify_content: JustifyContent::Center,
        align_self: AlignSelf::FlexEnd,
        ..default()
      },
      ..default()
    })
    .insert(TowerUIRoot) // Marker component
    .insert(Name::new("TowerButtons"))
    .with_children(|commands| { // Make the buttons children of the menu
      for i in TowerType::iter() {
        commands
          .spawn(ButtonBundle {
            style: Style {
              size: Size::new(Val::Px(85.), Val::Px(80.)),
              align_self: AlignSelf::Center,
              margin: UiRect {
                left: Val::Percent(2.),
                right: Val::Percent(2.),
                ..default()
              },
              ..default()
            },
            image: assets.get_button_asset(i).into(),
            ..default()
          })
          .insert(TowerButtonState {
            price: tower_stats.tower[&i].tower.price
          })
          .insert(i)
          .insert(Name::new("TowerButton"));
      }
    });
}