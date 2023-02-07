use bevy::prelude::*;
use crate::{GameAssets, MainCamera, spawn_tower, TowerType};
use strum::IntoEnumIterator;

pub struct TowerButtonPlugin;

impl Plugin for TowerButtonPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(generate_ui)
       .add_system(tower_button_interaction)
       .add_system(drag_sprite);
  }
}

// Marker component to despawn buttons in UI
#[derive(Component)]
pub struct TowerUIRoot;

#[derive(Component)]
pub struct SpriteFollower;

fn drag_sprite(
  mut commands: Commands,
  query: Query<(Entity, &TowerType), With<SpriteFollower>>,
  assets: Res<GameAssets>,
  mouse: Res<Input<MouseButton>>,
  windows: Res<Windows>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
  let window = windows.get_primary().unwrap();
  let (camera, camera_transform) = camera_query.single();
  for (entity, tower_type) in query.iter() {
    // Spawn the tower if user clicks with mouse button in a valid tower placement zone!!!
    if mouse.just_pressed(MouseButton::Left) {
      if let Some(screen_pos) = window.cursor_position() {
        // Convert cursor position from window/screen position to world position
      
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
      
        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        // Normalized device coordinates
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
      
        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
      
        // use it to convert ndc to world-space coordinates
        let mut world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        
        world_pos.z = 0.;
        
        commands.entity(entity).despawn_recursive();
        spawn_tower(&mut commands, *tower_type, &assets, world_pos);
      }
    } else if mouse.just_pressed(MouseButton::Right) || window.cursor_position().is_none() {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn tower_button_interaction(
  mut commands: Commands,
  assets: Res<GameAssets>,
  interaction: Query<(&Interaction, &TowerType), (Changed<Interaction>, With<Button>)>,
  mut images: Query<&mut UiImage>
) {
  for (interaction, tower_type) in &interaction {
    match interaction {
      Interaction::Clicked => {
        info!("Spawning: {tower_type} wizard");
        // Change button UI!!!
        for mut image in images.iter_mut() {
          image.0 = tower_type.get_button_asset(&assets, "press");
        }
        
        // Spawn component that alerts the drag_sprite() system that a button has been pressed
        // and it starts moving a sprite with the cursor until the tower is placed
        commands.spawn(SpriteFollower).insert(*tower_type);
      }
      Interaction::Hovered => {
        // Change button UI!!!
        for mut image in images.iter_mut() {
          image.0 = tower_type.get_button_asset(&assets, "hover");
        }
      }
      Interaction::None => { // Change button UI
        // Change button UI!!!
        for mut image in images.iter_mut() {
          image.0 = tower_type.get_button_asset(&assets, "normal");
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
              size: Size::new(Val::Px(80.), Val::Px(80.)),
              align_self: AlignSelf::FlexEnd, // Bottom of screen
              margin: UiRect::all(Val::Percent(2.0)),
              ..default()
            },
            image: i.get_button_asset(&assets, "normal").into(),
            ..default()
          })
          .insert(i);
      }
    });
}