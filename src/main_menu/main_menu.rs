use bevy::{app::AppExit, prelude::*};

use crate::assets::*;
use crate::main_menu::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(GameState::MainMenu)
        .with_system(spawn_main_menu))
      .add_system_set(
        SystemSet::on_update(GameState::MainMenu)
          .with_system(start_button_clicked)
          .with_system(exit_button_clicked),
      );
  }
}

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct ExitButton;

fn start_button_clicked(
  mut commands: Commands,
  interactions: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
  menu_root: Query<Entity, With<MenuUIRoot>>,
  mut game_state: ResMut<State<GameState>>,
) {
  for interaction in &interactions {
    if matches!(interaction, Interaction::Clicked) {
      let root_entity = menu_root.single();
      commands.entity(root_entity).despawn_recursive();
      
      game_state.set(GameState::Gameplay).unwrap();
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

fn spawn_main_menu(
  mut commands: Commands, assets: Res<GameAssets>, asset_server: Res<AssetServer>
) {
  let start_button = commands
    .spawn(ButtonBundle {
      style: Style {
        size: Size::new(Val::Px(570.), Val::Px(147.)),
        align_self: AlignSelf::Center,
        justify_content: JustifyContent::Center,
        margin: UiRect::all(Val::Percent(2.)),
        ..default()
      },
      image: assets.start_button.clone().into(),
      ..default()
    }).id();
  commands.entity(start_button).insert(StartButton);
  
  let exit_button = commands
    .spawn(ButtonBundle {
      style: Style {
        size: Size::new(Val::Px(570.), Val::Px(147.)),
        align_self: AlignSelf::Center,
        justify_content: JustifyContent::Center,
        margin: UiRect::all(Val::Percent(2.)),
        ..default()
      },
      image: assets.exit_button.clone().into(),
      ..default()
    }).id();
  commands.entity(exit_button).insert(ExitButton);
  
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
            font: asset_server.load("font/FiraSans-Bold.ttf"),
            font_size: 90.,
            color: Color::CYAN,
          },
        ),
        ..default()
      });
    })
    .add_child(start_button)
    .add_child(exit_button);
}