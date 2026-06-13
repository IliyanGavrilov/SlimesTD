use bevy::prelude::*;

use crate::assets::*;
use crate::main_menu::*;

pub struct VictoryPlugin;

impl Plugin for VictoryPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(spawn_victory_screen.in_schedule(OnEnter(GameState::Victory)))
      .add_system(menu_button_clicked.in_set(OnUpdate(GameState::Victory)))
      .add_system(cleanup_victory.in_schedule(OnExit(GameState::Victory)));
  }
}

#[derive(Component)]
struct VictoryUIRoot;

#[derive(Component)]
struct MainMenuButton;

fn spawn_victory_screen(mut commands: Commands, assets: Res<GameAssets>) {
  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
      },
      background_color: Color::rgba(0.0, 0.1, 0.0, 0.97).into(),
      ..default()
    })
    .insert(VictoryUIRoot)
    .insert(Name::new("VictoryScreen"))
    .with_children(|commands| {
      commands.spawn(TextBundle {
        style: Style {
          margin: UiRect::all(Val::Percent(3.)),
          ..default()
        },
        text: Text::from_section(
          "You Win!",
          TextStyle {
            font: assets.font.clone(),
            font_size: 100.,
            color: Color::YELLOW,
          },
        ),
        ..default()
      });

      commands
        .spawn(ButtonBundle {
          style: Style {
            size: Size::new(Val::Px(340.), Val::Px(90.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(2.)),
            ..default()
          },
          background_color: Color::DARK_GRAY.into(),
          ..default()
        })
        .insert(MainMenuButton)
        .with_children(|commands| {
          commands.spawn(TextBundle {
            text: Text::from_section(
              "Main Menu",
              TextStyle {
                font: assets.font.clone(),
                font_size: 40.,
                color: Color::WHITE,
              },
            ),
            ..default()
          });
        });
    });
}

fn menu_button_clicked(
  interactions: Query<&Interaction, (With<MainMenuButton>, Changed<Interaction>)>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  for interaction in &interactions {
    if matches!(interaction, Interaction::Clicked) {
      next_state.set(GameState::MainMenu);
    }
  }
}

fn cleanup_victory(mut commands: Commands, roots: Query<Entity, With<VictoryUIRoot>>) {
  for entity in &roots {
    commands.entity(entity).despawn_recursive();
  }
}
