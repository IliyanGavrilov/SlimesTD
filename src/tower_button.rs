use bevy::prelude::*;
use crate::{GameAssets, spawn_tower, TowerType};
use strum::IntoEnumIterator;

pub struct TowerButtonPlugin;

impl Plugin for TowerButtonPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(generate_ui)
       .add_system(tower_button_interaction);
  }
}

// Marker component to despawn buttons in UI
#[derive(Component)]
pub struct TowerUIRoot;

fn tower_button_interaction(
  // mut commands: Commands,
  // windows: Res<Windows>,
  // mouse: Res<Input<MouseButton>>,
  assets: Res<GameAssets>,
  interaction: Query<(&Interaction, &TowerType), (Changed<Interaction>, With<Button>)>,
  mut images: Query<&mut UiImage>
  //mut transforms: Query<&mut Transform>
) {
  //let window = windows.get_primary().unwrap();
  for (interaction, tower_type) in &interaction {
    //let mut image = images.get_mut(children[1]).unwrap();
    match interaction {
      Interaction::Clicked => {
        // Change button UI
        //image = assets.wizard_fire_button_press.clone().into();
    
        info!("Spawning: {tower_type} wizard");
    
        // Spawn asset that follows mouse until it is clicked
        // let sprite = commands.spawn(SpriteBundle {
        //   texture: assets.wizard_fire.clone(),
        //   transform:
        //   Transform::from_translation(window.cursor_position().unwrap().extend(0.)),
        //   ..default()
        // }).insert(ButtonClicked);
        //
        // let mut sprite_pos = transforms.get_mut(sprite.id()).unwrap();
  
        // loop {
        //   if mouse.just_pressed(MouseButton::Left) {
        //     if let Some(position) = window.cursor_position() {
        //       //spawn_tower(&mut commands, *tower_type, &assets, position.extend(0.));
        //     }
        //   }
        //   else if mouse.just_pressed(MouseButton::Right) ||
        //     window.cursor_position().is_none() {
        //     sprite.despawn_recursive();
        //     break
        //   }
        //
        //   sprite_pos.translation = window.cursor_position().unwrap().extend(0.);
        // }
    
        // Upon clicking the mouse, spawn the selected tower on the map
        // Spawn the tower if user clicks with mouse button in a valid tower placement zone!!!
        // if mouse.just_pressed(MouseButton::Left) { // Remove player money!!!
        //   if let Some(position) = window.cursor_position() {
        //     spawn_tower(&mut commands, *tower_type, &assets, position.extend(0.));
        //   }
        // }
        // If user clicks the right button or the mouse goes off the screen discard the selected tower
        // And stop the tower asset from following the mouse
        // else if mouse.just_pressed(MouseButton::Right) || window.cursor_position().is_none() {
        //
        // }
      }
      Interaction::Hovered => { // Change button UI
        for mut image in images.iter_mut() {
          image.0 = tower_type.get_button_asset(&assets, "hover");
        }
        //image = tower_type.get_button_asset(&assets, "hover");
        //interaction. image = assets.wizard_fire_button_hover.clone().into();
      }
      Interaction::None => { // Change button UI
        //image = assets.wizard_fire_button.clone().into();
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