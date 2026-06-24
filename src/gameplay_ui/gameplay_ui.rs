use bevy::{prelude::*, window::*};

use crate::assets::*;
use crate::gameplay_ui::*;
use crate::{AudioSettings, VolumeKind, spawn_volume_row};
use crate::{FullscreenToggleButton, GameData, GameState, VsyncToggleButton, Waves};

#[derive(Component)]
pub struct GameplayUIRoot;

#[derive(Component)]
pub struct HealthUI;

#[derive(Component)]
pub struct MoneyUI;

#[derive(Component)]
pub struct RoundUI;

#[derive(Component)]
struct InGameMenuButton;

#[derive(Component)]
pub struct InGameMenuRoot;

#[derive(Component)]
struct LeaveGameButton;

#[derive(Component)]
struct CloseMenuButton;

#[derive(Resource, Default)]
pub struct InGameMenuOpen(pub bool);

pub fn game_not_paused(menu: Res<InGameMenuOpen>) -> bool {
  !menu.0
}

pub struct GameplayUIPlugin;

impl Plugin for GameplayUIPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<InGameMenuOpen>()
      .add_system(spawn_gameplay_ui.in_schedule(OnEnter(GameState::Gameplay)))
      .add_system(spawn_in_game_menu.in_schedule(OnEnter(GameState::Gameplay)))
      .add_systems(
        (
          update_gameplay_ui,
          toggle_in_game_menu_key,
          in_game_menu_button_clicked,
          update_in_game_menu_visibility,
          leave_game_button_clicked,
          close_menu_button_clicked,
        )
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_system(cleanup_gameplay_ui.in_schedule(OnExit(GameState::Gameplay)))
      .add_system(cleanup_in_game_menu.in_schedule(OnExit(GameState::Gameplay)));
  }
}

fn update_gameplay_ui(
  player: Query<&Player>,
  base: Query<&Base>,
  game_data: Res<GameData>,
  waves: Res<Assets<Waves>>,
  mut money_ui: Query<&mut Text, (With<MoneyUI>, Without<HealthUI>, Without<RoundUI>)>,
  mut health_ui: Query<&mut Text, (With<HealthUI>, Without<RoundUI>)>,
  mut round_ui: Query<&mut Text, With<RoundUI>>,
) {
  let player = player.single();
  let base = base.single();
  let Some(waves) = waves.get(&game_data.enemy_waves) else {
    return;
  };
  let mut money = money_ui.single_mut();
  let mut health = health_ui.single_mut();
  let mut round = round_ui.single_mut();

  *money = Text::from_section(format!("{}", player.money), money.sections[0].style.clone());
  *health = Text::from_section(format!("{}", base.health), health.sections[0].style.clone());

  if waves.current < waves.waves.len() {
    *round = Text::from_section(
      format!("{}/{}", waves.current + 1, waves.waves.len()),
      round.sections[0].style.clone(),
    );
  }
}

fn toggle_in_game_menu_key(input: Res<Input<KeyCode>>, mut menu_open: ResMut<InGameMenuOpen>) {
  if input.just_pressed(KeyCode::Escape) {
    menu_open.0 = !menu_open.0;
  }
}

fn in_game_menu_button_clicked(
  interactions: Query<&Interaction, (With<InGameMenuButton>, Changed<Interaction>)>,
  mut menu_open: ResMut<InGameMenuOpen>,
) {
  for interaction in &interactions {
    if matches!(interaction, Interaction::Clicked) {
      menu_open.0 = !menu_open.0;
    }
  }
}

fn update_in_game_menu_visibility(
  menu_open: Res<InGameMenuOpen>,
  mut roots: Query<&mut Visibility, With<InGameMenuRoot>>,
) {
  if !menu_open.is_changed() {
    return;
  }
  for mut visibility in &mut roots {
    *visibility = if menu_open.0 {
      Visibility::Visible
    } else {
      Visibility::Hidden
    };
  }
}

fn leave_game_button_clicked(
  interactions: Query<&Interaction, (With<LeaveGameButton>, Changed<Interaction>)>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  for interaction in &interactions {
    if matches!(interaction, Interaction::Clicked) {
      next_state.set(GameState::MainMenu);
    }
  }
}

fn close_menu_button_clicked(
  interactions: Query<&Interaction, (With<CloseMenuButton>, Changed<Interaction>)>,
  mut menu_open: ResMut<InGameMenuOpen>,
) {
  for interaction in &interactions {
    if matches!(interaction, Interaction::Clicked) {
      menu_open.0 = false;
    }
  }
}

fn cleanup_gameplay_ui(mut commands: Commands, roots: Query<Entity, With<GameplayUIRoot>>) {
  for entity in &roots {
    commands.entity(entity).despawn_recursive();
  }
}

fn cleanup_in_game_menu(
  mut commands: Commands,
  roots: Query<Entity, With<InGameMenuRoot>>,
  mut menu_open: ResMut<InGameMenuOpen>,
) {
  menu_open.0 = false;
  for entity in &roots {
    commands.entity(entity).despawn_recursive();
  }
}

fn spawn_gameplay_ui(mut commands: Commands, assets: Res<GameAssets>) {
  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
        position_type: PositionType::Absolute,
        justify_content: JustifyContent::FlexStart,
        flex_direction: FlexDirection::Column,
        align_content: AlignContent::FlexStart,
        ..default()
      },
      ..default()
    })
    .insert(GameplayUIRoot)
    .with_children(|commands| {
      commands
        .spawn(NodeBundle {
          style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_self: AlignSelf::FlexStart,
            flex_direction: FlexDirection::Row,
            align_content: AlignContent::FlexStart,
            ..default()
          },
          ..default()
        })
        .insert(GameplayUIRoot)
        .with_children(|commands| {
          commands
            .spawn(ImageBundle {
              style: Style {
                margin: UiRect {
                  left: Val::Percent(2.5),
                  right: Val::Percent(0.25),
                  top: Val::Percent(2.5),
                  bottom: Val::Percent(2.5),
                },
                ..default()
              },
              image: assets.heart.clone().into(),
              ..default()
            })
            .insert(Name::new("HeartImage"));

          commands
            .spawn(TextBundle {
              style: Style {
                margin: UiRect {
                  left: Val::Percent(0.25),
                  right: Val::Percent(4.5),
                  top: Val::Percent(2.5),
                  bottom: Val::Percent(2.5),
                },
                ..default()
              },
              text: Text::from_section(
                "",
                TextStyle {
                  font: assets.font.clone(),
                  font_size: 36.,
                  color: Color::WHITE,
                },
              ),
              ..default()
            })
            .insert(HealthUI)
            .insert(Name::new("Health"));

          commands
            .spawn(ImageBundle {
              style: Style {
                margin: UiRect {
                  left: Val::Percent(4.5),
                  right: Val::Percent(0.25),
                  top: Val::Percent(2.5),
                  bottom: Val::Percent(2.5),
                },
                ..default()
              },
              image: assets.coin.clone().into(),
              ..default()
            })
            .insert(Name::new("MoneyImage"));

          commands
            .spawn(TextBundle {
              style: Style {
                margin: UiRect {
                  left: Val::Percent(0.3),
                  right: Val::Percent(4.5),
                  top: Val::Percent(2.5),
                  bottom: Val::Percent(2.5),
                },
                ..default()
              },
              text: Text::from_section(
                "",
                TextStyle {
                  font: assets.font.clone(),
                  font_size: 36.,
                  color: Color::WHITE,
                },
              ),
              ..default()
            })
            .insert(MoneyUI)
            .insert(Name::new("Money"));

          commands
            .spawn(TextBundle {
              style: Style {
                margin: UiRect {
                  left: Val::Auto,
                  right: Val::Px(10.),
                  top: Val::Percent(2.5),
                  bottom: Val::Percent(2.5),
                },
                ..default()
              },
              text: Text::from_section(
                "",
                TextStyle {
                  font: assets.font.clone(),
                  font_size: 36.,
                  color: Color::WHITE,
                },
              ),
              ..default()
            })
            .insert(RoundUI)
            .insert(Name::new("Round"));

          commands
            .spawn(ButtonBundle {
              style: Style {
                margin: UiRect {
                  left: Val::Px(6.),
                  right: Val::Percent(1.),
                  top: Val::Percent(0.5),
                  bottom: Val::Percent(0.5),
                },
                size: Size::new(Val::Px(90.), Val::Px(38.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
              },
              background_color: Color::rgba(0.15, 0.15, 0.15, 0.9).into(),
              ..default()
            })
            .insert(InGameMenuButton)
            .insert(Name::new("MenuButton"))
            .with_children(|commands| {
              commands.spawn(TextBundle {
                text: Text::from_section(
                  "Menu",
                  TextStyle {
                    font: assets.font.clone(),
                    font_size: 22.,
                    color: Color::WHITE,
                  },
                ),
                ..default()
              });
            });
        });
    })
    .insert(Name::new("GameplayUI"));
}

fn spawn_in_game_menu(
  mut commands: Commands,
  assets: Res<GameAssets>,
  windows: Query<&Window>,
  audio_settings: Res<AudioSettings>,
) {
  let window = windows.single();
  let vsync_on = matches!(window.present_mode, PresentMode::AutoVsync);
  let fullscreen_on = !matches!(window.mode, WindowMode::Windowed);
  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
        position_type: PositionType::Absolute,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
      },
      background_color: Color::rgba(0.05, 0.05, 0.05, 0.88).into(),
      visibility: Visibility::Hidden,
      ..default()
    })
    .insert(InGameMenuRoot)
    .insert(Name::new("InGameMenu"))
    .with_children(|commands| {
      commands.spawn(TextBundle {
        style: Style {
          margin: UiRect::bottom(Val::Percent(4.)),
          ..default()
        },
        text: Text::from_section(
          "Menu",
          TextStyle {
            font: assets.font.clone(),
            font_size: 60.,
            color: Color::CYAN,
          },
        ),
        ..default()
      });

      spawn_menu_toggle_row(
        commands,
        &assets,
        "VSync",
        "[V]",
        vsync_on,
        VsyncToggleButton,
      );
      spawn_menu_toggle_row(
        commands,
        &assets,
        "Fullscreen",
        "[F11]",
        fullscreen_on,
        FullscreenToggleButton,
      );

      spawn_volume_row(
        commands,
        &assets,
        "Master",
        VolumeKind::Master,
        audio_settings.master,
      );
      spawn_volume_row(
        commands,
        &assets,
        "Music",
        VolumeKind::Music,
        audio_settings.music,
      );
      spawn_volume_row(
        commands,
        &assets,
        "SFX",
        VolumeKind::Sfx,
        audio_settings.sfx,
      );

      commands
        .spawn(ButtonBundle {
          style: Style {
            size: Size::new(Val::Px(280.), Val::Px(65.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::top(Val::Percent(5.)),
            ..default()
          },
          background_color: Color::rgb(0.5, 0.12, 0.12).into(),
          ..default()
        })
        .insert(LeaveGameButton)
        .with_children(|commands| {
          commands.spawn(TextBundle {
            text: Text::from_section(
              "Leave to Menu",
              TextStyle {
                font: assets.font.clone(),
                font_size: 30.,
                color: Color::WHITE,
              },
            ),
            ..default()
          });
        });

      commands
        .spawn(ButtonBundle {
          style: Style {
            size: Size::new(Val::Px(280.), Val::Px(65.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::top(Val::Percent(2.)),
            ..default()
          },
          background_color: Color::DARK_GRAY.into(),
          ..default()
        })
        .insert(CloseMenuButton)
        .with_children(|commands| {
          commands.spawn(TextBundle {
            text: Text::from_section(
              "Close  [Esc]",
              TextStyle {
                font: assets.font.clone(),
                font_size: 30.,
                color: Color::WHITE,
              },
            ),
            ..default()
          });
        });
    });
}

fn toggle_color(on: bool) -> Color {
  if on {
    Color::rgb(0.15, 0.45, 0.15)
  } else {
    Color::rgb(0.25, 0.25, 0.25)
  }
}

fn spawn_menu_toggle_row(
  commands: &mut ChildBuilder,
  assets: &GameAssets,
  label: &str,
  key_hint: &str,
  is_on: bool,
  marker: impl Component,
) {
  commands
    .spawn(NodeBundle {
      style: Style {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        size: Size::new(Val::Px(480.), Val::Px(44.)),
        margin: UiRect::vertical(Val::Px(6.)),
        ..default()
      },
      ..default()
    })
    .with_children(|commands| {
      commands
        .spawn(NodeBundle {
          style: Style {
            size: Size::new(Val::Px(200.), Val::Percent(100.)),
            align_items: AlignItems::Center,
            ..default()
          },
          ..default()
        })
        .with_children(|commands| {
          commands.spawn(TextBundle {
            text: Text::from_section(
              label,
              TextStyle {
                font: assets.font.clone(),
                font_size: 28.,
                color: Color::WHITE,
              },
            ),
            ..default()
          });
        });
      commands
        .spawn(NodeBundle {
          style: Style {
            size: Size::new(Val::Px(130.), Val::Percent(100.)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
          },
          ..default()
        })
        .with_children(|commands| {
          commands.spawn(TextBundle {
            text: Text::from_section(
              key_hint,
              TextStyle {
                font: assets.font.clone(),
                font_size: 18.,
                color: Color::rgb(0.55, 0.55, 0.55),
              },
            ),
            ..default()
          });
        });
      commands
        .spawn(ButtonBundle {
          style: Style {
            size: Size::new(Val::Px(72.), Val::Px(32.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
          },
          background_color: toggle_color(is_on).into(),
          ..default()
        })
        .insert(marker)
        .with_children(|commands| {
          commands.spawn(TextBundle {
            text: Text::from_section(
              if is_on { "ON" } else { "OFF" },
              TextStyle {
                font: assets.font.clone(),
                font_size: 20.,
                color: Color::WHITE,
              },
            ),
            ..default()
          });
        });
    });
}
