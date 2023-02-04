use bevy::prelude::*;
use crate::{GameAssets, TowerType};
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

fn tower_button_interaction(interaction: Query<(&Interaction, &TowerType), Changed<Interaction>>) {
  for (interaction, tower_type) in &interaction {
    match interaction {
      Interaction::Clicked => {
        // Change button UI
        //image = assets.wizard_fire_button_press.clone().into();
        
        info!("Spawning: {tower_type} wizard");
        
        // Spawn asset that follows mouse until it is clicked
        
        
        // Upon clicking the mouse, spawn the selected tower on the map
        
      }
      Interaction::Hovered => { // Change button UI
        //image = assets.wizard_fire_button_hover.clone().into();
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
            image: i.path(&assets).into(),
            ..default()
          })
          .insert(i);
      }
    });
}