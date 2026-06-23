use bevy::{prelude::*, window::*};

use crate::assets::*;
use crate::main_menu::*;
use crate::AudioSettings;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(SettingsReturnState(GameState::MainMenu))
      .add_system(spawn_settings_screen.in_schedule(OnEnter(GameState::Settings)))
      .add_system(settings_back_button_clicked.in_set(OnUpdate(GameState::Settings)))
      .add_system(cleanup_settings_screen.in_schedule(OnExit(GameState::Settings)))
      .add_system(toggle_vsync)
      .add_system(toggle_fullscreen)
      .add_system(vsync_toggle_button_clicked)
      .add_system(fullscreen_toggle_button_clicked)
      // Ungated so it also serves the in-game pause menu's volume sliders.
      .add_system(volume_slider_drag);
  }
}

#[derive(Resource)]
pub struct SettingsReturnState(pub GameState);

#[derive(Component)]
struct SettingsUIRoot;

#[derive(Component)]
struct SettingsBackButton;

#[derive(Component)]
pub struct VsyncToggleButton;

#[derive(Component)]
pub struct FullscreenToggleButton;

#[derive(Clone, Copy, PartialEq)]
pub enum VolumeKind {
  Master,
  Music,
  Sfx,
}

/// Draggable slider track for one volume. Carries `Interaction` (it's a button)
/// so we can detect press/drag, plus `Node`/`GlobalTransform` for cursor math.
#[derive(Component, Clone, Copy)]
pub struct VolumeSlider {
  kind: VolumeKind,
}

/// The coloured fill inside a slider track; its width mirrors the value.
#[derive(Component, Clone, Copy)]
pub struct VolumeSliderFill {
  kind: VolumeKind,
}

/// The "60%" text that mirrors a volume value.
#[derive(Component, Clone, Copy)]
pub struct VolumeValueText {
  kind: VolumeKind,
}

const SLIDER_WIDTH: f32 = 200.;

fn volume_percent(value: f32) -> String {
  format!("{}%", (value * 100.).round() as i32)
}

fn spawn_settings_screen(
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
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
      },
      background_color: Color::rgb(0.1, 0.1, 0.1).into(),
      ..default()
    })
    .insert(SettingsUIRoot)
    .insert(Name::new("SettingsScreen"))
    .with_children(|commands| {
      commands.spawn(TextBundle {
        style: Style {
          margin: UiRect::bottom(Val::Percent(5.)),
          ..default()
        },
        text: Text::from_section(
          "Settings",
          TextStyle {
            font: assets.font.clone(),
            font_size: 70.,
            color: Color::CYAN,
          },
        ),
        ..default()
      });

      spawn_toggle_row(commands, &assets, "VSync", "[V]", vsync_on, VsyncToggleButton);
      spawn_toggle_row(commands, &assets, "Fullscreen", "[F11]", fullscreen_on, FullscreenToggleButton);

      spawn_volume_row(commands, &assets, "Master", VolumeKind::Master, audio_settings.master);
      spawn_volume_row(commands, &assets, "Music", VolumeKind::Music, audio_settings.music);
      spawn_volume_row(commands, &assets, "SFX", VolumeKind::Sfx, audio_settings.sfx);

      commands
        .spawn(ButtonBundle {
          style: Style {
            size: Size::new(Val::Px(220.), Val::Px(65.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::top(Val::Percent(6.)),
            ..default()
          },
          background_color: Color::DARK_GRAY.into(),
          ..default()
        })
        .insert(SettingsBackButton)
        .with_children(|commands| {
          commands.spawn(TextBundle {
            text: Text::from_section(
              "Back",
              TextStyle {
                font: assets.font.clone(),
                font_size: 36.,
                color: Color::WHITE,
              },
            ),
            ..default()
          });
        });
    });
}

fn toggle_color(on: bool) -> Color {
  if on { Color::rgb(0.15, 0.45, 0.15) } else { Color::rgb(0.25, 0.25, 0.25) }
}

fn spawn_toggle_row(
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
        size: Size::new(Val::Px(520.), Val::Px(50.)),
        margin: UiRect::vertical(Val::Px(8.)),
        ..default()
      },
      ..default()
    })
    .with_children(|commands| {
      // Fixed-width label column
      commands.spawn(NodeBundle {
        style: Style {
          size: Size::new(Val::Px(220.), Val::Percent(100.)),
          align_items: AlignItems::Center,
          ..default()
        },
        ..default()
      }).with_children(|commands| {
        commands.spawn(TextBundle {
          text: Text::from_section(
            label,
            TextStyle { font: assets.font.clone(), font_size: 32., color: Color::WHITE },
          ),
          ..default()
        });
      });
      // Fixed-width key hint column
      commands.spawn(NodeBundle {
        style: Style {
          size: Size::new(Val::Px(140.), Val::Percent(100.)),
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          ..default()
        },
        ..default()
      }).with_children(|commands| {
        commands.spawn(TextBundle {
          text: Text::from_section(
            key_hint,
            TextStyle { font: assets.font.clone(), font_size: 20., color: Color::rgb(0.55, 0.55, 0.55) },
          ),
          ..default()
        });
      });
      // Toggle button
      commands
        .spawn(ButtonBundle {
          style: Style {
            size: Size::new(Val::Px(80.), Val::Px(38.)),
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
              TextStyle { font: assets.font.clone(), font_size: 22., color: Color::WHITE },
            ),
            ..default()
          });
        });
    });
}

pub fn spawn_volume_row(
  commands: &mut ChildBuilder,
  assets: &GameAssets,
  label: &str,
  kind: VolumeKind,
  value: f32,
) {
  commands
    .spawn(NodeBundle {
      style: Style {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        size: Size::new(Val::Px(520.), Val::Px(50.)),
        margin: UiRect::vertical(Val::Px(8.)),
        ..default()
      },
      ..default()
    })
    .with_children(|commands| {
      // Fixed-width label column
      commands
        .spawn(NodeBundle {
          style: Style {
            size: Size::new(Val::Px(220.), Val::Percent(100.)),
            align_items: AlignItems::Center,
            ..default()
          },
          ..default()
        })
        .with_children(|commands| {
          commands.spawn(TextBundle {
            text: Text::from_section(
              label,
              TextStyle { font: assets.font.clone(), font_size: 32., color: Color::WHITE },
            ),
            ..default()
          });
        });
      // Slider track (clickable + draggable)
      commands
        .spawn((
          ButtonBundle {
            style: Style {
              size: Size::new(Val::Px(SLIDER_WIDTH), Val::Px(20.)),
              align_items: AlignItems::Center,
              ..default()
            },
            background_color: Color::rgb(0.22, 0.22, 0.22).into(),
            ..default()
          },
          VolumeSlider { kind },
        ))
        .with_children(|commands| {
          // Coloured fill, width = value%.
          commands.spawn((
            NodeBundle {
              style: Style {
                size: Size::new(Val::Percent(value * 100.), Val::Percent(100.)),
                ..default()
              },
              background_color: Color::rgb(0.2, 0.65, 0.3).into(),
              ..default()
            },
            VolumeSliderFill { kind },
          ));
        });
      // Value column
      commands
        .spawn(NodeBundle {
          style: Style {
            size: Size::new(Val::Px(80.), Val::Percent(100.)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
          },
          ..default()
        })
        .with_children(|commands| {
          commands.spawn((
            TextBundle {
              text: Text::from_section(
                volume_percent(value),
                TextStyle { font: assets.font.clone(), font_size: 28., color: Color::WHITE },
              ),
              ..default()
            },
            VolumeValueText { kind },
          ));
        });
    });
}

/// Click or drag anywhere on a slider track to set that volume. Runs ungated so
/// it serves both the main-menu Settings screen and the in-game pause menu.
fn volume_slider_drag(
  windows: Query<&Window>,
  sliders: Query<(&Interaction, &VolumeSlider, &Node, &GlobalTransform)>,
  mut audio_settings: ResMut<AudioSettings>,
  mut fills: Query<(&mut Style, &VolumeSliderFill)>,
  mut texts: Query<(&mut Text, &VolumeValueText)>,
) {
  let Ok(window) = windows.get_single() else { return; };
  let Some(cursor) = window.cursor_position() else { return; };

  for (interaction, slider, node, transform) in &sliders {
    if !matches!(interaction, Interaction::Clicked) {
      continue;
    }
    // Only the X axis matters; UI X and cursor X share the same left origin.
    let width = node.size().x;
    if width <= 0. {
      continue;
    }
    let left = transform.translation().x - width / 2.;
    let value = ((cursor.x - left) / width).clamp(0., 1.);

    match slider.kind {
      VolumeKind::Master => audio_settings.master = value,
      VolumeKind::Music => audio_settings.music = value,
      VolumeKind::Sfx => audio_settings.sfx = value,
    }

    for (mut style, fill) in &mut fills {
      if fill.kind == slider.kind {
        style.size.width = Val::Percent(value * 100.);
      }
    }
    for (mut text, value_text) in &mut texts {
      if value_text.kind == slider.kind {
        text.sections[0].value = volume_percent(value);
      }
    }
  }
}

fn vsync_toggle_button_clicked(
  mut interactions: Query<
    (&Interaction, &Children, &mut BackgroundColor),
    (With<VsyncToggleButton>, Changed<Interaction>),
  >,
  mut windows: Query<&mut Window>,
  mut texts: Query<&mut Text>,
) {
  for (interaction, children, mut color) in &mut interactions {
    if matches!(interaction, Interaction::Clicked) {
      let mut window = windows.single_mut();
      let now_on = !matches!(window.present_mode, PresentMode::AutoVsync);
      window.present_mode = if now_on { PresentMode::AutoVsync } else { PresentMode::AutoNoVsync };
      *color = toggle_color(now_on).into();
      for &child in children.iter() {
        if let Ok(mut text) = texts.get_mut(child) {
          text.sections[0].value = if now_on { "ON".to_string() } else { "OFF".to_string() };
        }
      }
    }
  }
}

fn fullscreen_toggle_button_clicked(
  mut interactions: Query<
    (&Interaction, &Children, &mut BackgroundColor),
    (With<FullscreenToggleButton>, Changed<Interaction>),
  >,
  mut windows: Query<&mut Window>,
  mut texts: Query<&mut Text>,
) {
  for (interaction, children, mut color) in &mut interactions {
    if matches!(interaction, Interaction::Clicked) {
      let mut window = windows.single_mut();
      let now_on = matches!(window.mode, WindowMode::Windowed);
      window.mode = if now_on { WindowMode::BorderlessFullscreen } else { WindowMode::Windowed };
      *color = toggle_color(now_on).into();
      for &child in children.iter() {
        if let Ok(mut text) = texts.get_mut(child) {
          text.sections[0].value = if now_on { "ON".to_string() } else { "OFF".to_string() };
        }
      }
    }
  }
}

fn settings_back_button_clicked(
  interactions: Query<&Interaction, (With<SettingsBackButton>, Changed<Interaction>)>,
  mut next_state: ResMut<NextState<GameState>>,
  return_state: Res<SettingsReturnState>,
) {
  for interaction in &interactions {
    if matches!(interaction, Interaction::Clicked) {
      next_state.set(return_state.0.clone());
    }
  }
}

fn cleanup_settings_screen(mut commands: Commands, roots: Query<Entity, With<SettingsUIRoot>>) {
  for entity in &roots {
    commands.entity(entity).despawn_recursive();
  }
}

fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
  if input.just_pressed(KeyCode::V) {
    let mut window = windows.single_mut();
    window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
      PresentMode::AutoNoVsync
    } else {
      PresentMode::AutoVsync
    };
  }
}

fn toggle_fullscreen(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
  if ((input.pressed(KeyCode::LAlt) || input.pressed(KeyCode::RAlt))
    && input.just_pressed(KeyCode::Return))
    || input.just_pressed(KeyCode::F11)
  {
    let mut window = windows.single_mut();
    window.mode = if matches!(window.mode, WindowMode::Windowed) {
      WindowMode::BorderlessFullscreen
    } else {
      WindowMode::Windowed
    };
  }
}
