use bevy::prelude::*;

use crate::assets::*;
use crate::main_menu::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(spawn_game_over_screen.in_schedule(OnEnter(GameState::GameOver)))
      .add_system(menu_button_clicked.in_set(OnUpdate(GameState::GameOver)))
      .add_system(cleanup_game_over.in_schedule(OnExit(GameState::GameOver)));
  }
}

#[derive(Component)]
struct GameOverUIRoot;

#[derive(Component)]
struct MainMenuButton;

fn spawn_game_over_screen(mut commands: Commands, assets: Res<GameAssets>) {
  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
      },
      background_color: Color::rgba(0.05, 0.05, 0.05, 0.97).into(),
      ..default()
    })
    .insert(GameOverUIRoot)
    .insert(Name::new("GameOverScreen"))
    .with_children(|commands| {
      commands.spawn(TextBundle {
        style: Style {
          margin: UiRect::all(Val::Percent(3.)),
          ..default()
        },
        text: Text::from_section(
          "Game Over",
          TextStyle {
            font: assets.font.clone(),
            font_size: 100.,
            color: Color::RED,
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

fn cleanup_game_over(mut commands: Commands, roots: Query<Entity, With<GameOverUIRoot>>) {
  for entity in &roots {
    commands.entity(entity).despawn_recursive();
  }
}
