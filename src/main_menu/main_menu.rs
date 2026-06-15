use bevy::{app::AppExit, prelude::*};

use crate::assets::*;
use crate::main_menu::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(spawn_main_menu.in_schedule(OnEnter(GameState::MainMenu)))
      .add_system(cleanup_main_menu.in_schedule(OnExit(GameState::MainMenu)))
      .add_systems(
        (start_button_clicked, settings_button_clicked, exit_button_clicked)
          .in_set(OnUpdate(GameState::MainMenu)),
      );
  }
}

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct ExitButton;

fn cleanup_main_menu(mut commands: Commands, roots: Query<Entity, With<MenuUIRoot>>) {
  for entity in &roots {
    commands.entity(entity).despawn_recursive();
  }
}

fn start_button_clicked(
  interactions: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  for interaction in &interactions {
    if matches!(interaction, Interaction::Clicked) {
      next_state.set(GameState::MapSelection);
    }
  }
}

fn settings_button_clicked(
  interactions: Query<&Interaction, (With<SettingsButton>, Changed<Interaction>)>,
  mut next_state: ResMut<NextState<GameState>>,
  mut return_state: ResMut<SettingsReturnState>,
) {
  for interaction in &interactions {
    if matches!(interaction, Interaction::Clicked) {
      return_state.0 = GameState::MainMenu;
      next_state.set(GameState::Settings);
    }
  }
}

fn exit_button_clicked(
  interactions: Query<&Interaction, (With<ExitButton>, Changed<Interaction>)>,
  mut exit: EventWriter<AppExit>,
) {
  for interaction in &interactions {
    if matches!(interaction, Interaction::Clicked) {
      exit.send(AppExit);
    }
  }
}

fn spawn_main_menu(mut commands: Commands, assets: Res<GameAssets>) {
  let start_button = commands
    .spawn(ButtonBundle {
      style: image_button_style(),
      image: assets.start_button.clone().into(),
      ..default()
    })
    .insert(StartButton)
    .id();

  let settings_button = commands
    .spawn(ButtonBundle {
      style: image_button_style(),
      image: assets.settings_button.clone().into(),
      ..default()
    })
    .insert(SettingsButton)
    .id();

  let exit_button = commands
    .spawn(ButtonBundle {
      style: image_button_style(),
      image: assets.exit_button.clone().into(),
      ..default()
    })
    .insert(ExitButton)
    .id();

  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        ..default()
      },
      ..default()
    })
    .insert(MenuUIRoot)
    .with_children(|commands| {
      commands.spawn(TextBundle {
        style: Style {
          align_self: AlignSelf::Center,
          margin: UiRect::all(Val::Percent(3.)),
          ..default()
        },
        text: Text::from_section(
          "Slimes Tower Defense",
          TextStyle {
            font: assets.font.clone(),
            font_size: 90.,
            color: Color::CYAN,
          },
        ),
        ..default()
      });
    })
    .add_child(start_button)
    .add_child(settings_button)
    .add_child(exit_button);
}

fn image_button_style() -> Style {
  Style {
    size: Size::new(Val::Px(570.), Val::Px(147.)),
    align_self: AlignSelf::Center,
    justify_content: JustifyContent::Center,
    margin: UiRect::all(Val::Percent(2.)),
    ..default()
  }
}
