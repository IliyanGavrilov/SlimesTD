use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{persistence, GameAssets, GameState, Player, Tower, TowerButtonState, TowerUpgradeButton};

/// Persistence key (`progress.ron`). Persisted so the tutorial only auto-runs on
/// the first ever launch (recompiling doesn't reset it). Delete the file to
/// re-test, or use the "How to Play" button.
const PROGRESS_KEY: &str = "progress";

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(TutorialProgress::load())
      .init_resource::<TutorialState>()
      .init_resource::<TutorialRequest>()
      .add_system(start_tutorial.in_schedule(OnEnter(GameState::Gameplay)))
      .add_system(update_tutorial.in_set(OnUpdate(GameState::Gameplay)))
      .add_system(tutorial_button_glow.in_set(OnUpdate(GameState::Gameplay)))
      .add_system(cleanup_tutorial.in_schedule(OnExit(GameState::Gameplay)));
  }
}

/// Live tutorial run-state. `active` also gates wave spawning (see `spawn_waves`).
#[derive(Resource, Default)]
pub struct TutorialState {
  pub active: bool,
  pub step: usize,
  /// One-time bonus gold granted so the mandatory upgrade step is affordable.
  granted: bool,
}

/// Set by the main-menu "How to Play" button to force a replay next game.
#[derive(Resource, Default)]
pub struct TutorialRequest {
  pub force_replay: bool,
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct TutorialProgress {
  pub completed: bool,
}

impl TutorialProgress {
  fn load() -> Self {
    persistence::load_or_default(PROGRESS_KEY)
  }

  fn save(&self) {
    persistence::save(PROGRESS_KEY, self);
  }
}

/// How a step advances.
enum Gate {
  /// Advance on Space.
  Continue,
  /// Advance once the player has placed a tower (or Space to skip).
  PlaceTower,
  /// Advance once the player has bought any upgrade (or Space to skip).
  Upgrade,
}

struct Step {
  text: &'static str,
  gate: Gate,
}

const STEPS: &[Step] = &[
  Step {
    text: "Slimes march along the path to your base.\nStop them before they break through.",
    gate: Gate::Continue,
  },
  Step {
    text: "Pick a tower from the bottom bar\n(click it or press 0-9),\nthen click a tile to place it.",
    gate: Gate::PlaceTower,
  },
  Step {
    text: "Towers auto-fire at slimes in range.\nEach type adds an effect: poison, slow,\nsplash, stun, knockback or chain.",
    gate: Gate::Continue,
  },
  Step {
    text: "Click a placed tower to open its panel.\nBuy an upgrade on the right (3 paths).\nSell with Backspace, switch aim with Tab.",
    gate: Gate::Upgrade,
  },
  Step {
    text: "Killing slimes earns gold.\nFarms add income every wave.",
    gate: Gate::Continue,
  },
  Step {
    text: "Press Esc for the menu and settings.\nWaves get tougher. Good luck!",
    gate: Gate::Continue,
  },
];

#[derive(Component)]
struct TutorialHintRoot;

#[derive(Component)]
struct TutorialHintText;

/// "Step X/N" label, top-left of the hint bar (fixed spot every step).
#[derive(Component)]
struct TutorialStepCounter;

/// Action prompt, bottom of the hint bar (fixed spot every step).
#[derive(Component)]
struct TutorialPrompt;

#[derive(Component)]
struct TutorialSkipButton;

/// Pulsing overlay child placed on each button a gated step wants the player to
/// click (tower buttons for the place step, upgrade buttons for the upgrade step).
#[derive(Component)]
struct TutorialGlow;

/// Minimum gold while the tutorial runs, so place + upgrade is always affordable.
const TUTORIAL_MIN_GOLD: usize = 500;

fn start_tutorial(
  mut commands: Commands,
  assets: Res<GameAssets>,
  progress: Res<TutorialProgress>,
  mut request: ResMut<TutorialRequest>,
  mut state: ResMut<TutorialState>,
) {
  let activate = !progress.completed || request.force_replay;
  request.force_replay = false;
  state.active = activate;
  state.step = 0;
  state.granted = false;

  if activate {
    spawn_hint_bar(&mut commands, &assets);
  }
}

#[allow(clippy::too_many_arguments)]
fn update_tutorial(
  mut state: ResMut<TutorialState>,
  mut progress: ResMut<TutorialProgress>,
  keys: Res<Input<KeyCode>>,
  mouse: Res<Input<MouseButton>>,
  towers: Query<&Tower>,
  mut player: Query<&mut Player>,
  skip_button: Query<&Interaction, (With<TutorialSkipButton>, Changed<Interaction>)>,
  mut hint_text: Query<
    &mut Text,
    (
      With<TutorialHintText>,
      Without<TutorialStepCounter>,
      Without<TutorialPrompt>,
    ),
  >,
  mut counter_text: Query<
    &mut Text,
    (
      With<TutorialStepCounter>,
      Without<TutorialHintText>,
      Without<TutorialPrompt>,
    ),
  >,
  mut prompt_text: Query<
    &mut Text,
    (
      With<TutorialPrompt>,
      Without<TutorialHintText>,
      Without<TutorialStepCounter>,
    ),
  >,
  mut commands: Commands,
  roots: Query<Entity, Or<(With<TutorialHintRoot>, With<TutorialGlow>)>>,
) {
  if !state.active {
    return;
  }

  // One-time gold grant so placing a tower AND buying an upgrade is affordable.
  if !state.granted {
    if let Ok(mut p) = player.get_single_mut() {
      p.money = p.money.max(TUTORIAL_MIN_GOLD);
    }
    state.granted = true;
  }

  // The Skip button bails the whole tutorial; individual action steps are otherwise
  // mandatory (Space only advances informational steps).
  if skip_button.iter().any(|i| matches!(i, Interaction::Clicked)) {
    finish(&mut state, &mut progress, &mut commands, &roots);
    return;
  }

  let step = &STEPS[state.step];
  let advance = match step.gate {
    Gate::Continue => {
      keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left)
    }
    Gate::PlaceTower => towers.iter().count() >= 1,
    Gate::Upgrade => towers
      .iter()
      .any(|t| t.upgrades.upgrades.iter().any(|&n| n > 0)),
  };

  if advance {
    state.step += 1;
    if state.step >= STEPS.len() {
      finish(&mut state, &mut progress, &mut commands, &roots);
      return;
    }
  }

  // Fixed-layout hint bar: counter (top), body (middle), prompt (bottom) - each
  // in the same spot every step.
  let step = &STEPS[state.step];
  if let Ok(mut text) = counter_text.get_single_mut() {
    text.sections[0].value = format!("Step {}/{}", state.step + 1, STEPS.len());
  }
  if let Ok(mut text) = hint_text.get_single_mut() {
    text.sections[0].value = step.text.to_string();
  }
  if let Ok(mut text) = prompt_text.get_single_mut() {
    text.sections[0].value = match step.gate {
      Gate::Continue => "[Space] / Click to Continue".to_string(),
      Gate::PlaceTower => "Build a tower to continue".to_string(),
      Gate::Upgrade => "Buy an upgrade to continue".to_string(),
    };
  }
}

/// Pulsing glow placed directly on the buttons the current gated step wants
/// clicked: tower buttons during the place step, upgrade buttons during the
/// upgrade step. Rebuilt each frame (cheap, only a handful of buttons) so it
/// follows the upgrade buttons that only exist once a tower is selected.
fn tutorial_button_glow(
  mut commands: Commands,
  state: Res<TutorialState>,
  time: Res<Time>,
  glows: Query<Entity, With<TutorialGlow>>,
  tower_buttons: Query<(&Node, &GlobalTransform), With<TowerButtonState>>,
  upgrade_buttons: Query<(&Node, &GlobalTransform), With<TowerUpgradeButton>>,
) {
  // Clear last frame's glows. These are standalone (not children of the buttons),
  // so the tutorial fully owns them - tearing down a tower's UI can't despawn them
  // out from under us, which previously caused a PushChildren panic.
  for entity in &glows {
    commands.entity(entity).despawn_recursive();
  }

  if !state.active {
    return;
  }

  // Cyan reads clearly over the yellow-tinted tower/upgrade buttons.
  let pulse = 0.25 + 0.3 * (time.elapsed_seconds() * 5.0).sin().mul_add(0.5, 0.5);
  let color: BackgroundColor = Color::rgba(0.1, 0.85, 1.0, pulse).into();

  let mut spawn_glow = |node: &Node, transform: &GlobalTransform| {
    let size = node.size();
    if size.x <= 0.0 || size.y <= 0.0 {
      return; // Not laid out yet.
    }
    // UI GlobalTransform is the node centre (logical px, origin top-left). Place an
    // absolute overlay of the same size over it.
    let center = transform.translation();
    commands.spawn((
      NodeBundle {
        style: Style {
          position_type: PositionType::Absolute,
          position: UiRect {
            left: Val::Px(center.x - size.x / 2.0),
            top: Val::Px(center.y - size.y / 2.0),
            ..default()
          },
          size: Size::new(Val::Px(size.x), Val::Px(size.y)),
          ..default()
        },
        background_color: color,
        // Let clicks reach the button underneath.
        focus_policy: bevy::ui::FocusPolicy::Pass,
        z_index: ZIndex::Global(30),
        ..default()
      },
      TutorialGlow,
      Name::new("TutorialGlow"),
    ));
  };

  match STEPS[state.step].gate {
    Gate::PlaceTower => {
      for (node, transform) in &tower_buttons {
        spawn_glow(node, transform);
      }
    }
    Gate::Upgrade => {
      for (node, transform) in &upgrade_buttons {
        spawn_glow(node, transform);
      }
    }
    Gate::Continue => {}
  }
}

fn finish(
  state: &mut TutorialState,
  progress: &mut TutorialProgress,
  commands: &mut Commands,
  roots: &Query<Entity, Or<(With<TutorialHintRoot>, With<TutorialGlow>)>>,
) {
  state.active = false;
  if !progress.completed {
    progress.completed = true;
    progress.save();
  }
  for entity in roots {
    commands.entity(entity).despawn_recursive();
  }
}

fn cleanup_tutorial(
  mut commands: Commands,
  mut state: ResMut<TutorialState>,
  roots: Query<Entity, Or<(With<TutorialHintRoot>, With<TutorialGlow>)>>,
) {
  state.active = false;
  for entity in &roots {
    commands.entity(entity).despawn_recursive();
  }
}

fn spawn_hint_bar(commands: &mut Commands, assets: &GameAssets) {
  // Fixed-height column so the three zones (counter / body / prompt) sit in the
  // same spot every step regardless of how long the body text is.
  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size::new(Val::Px(560.), Val::Px(190.)),
        position_type: PositionType::Absolute,
        position: UiRect {
          top: Val::Percent(13.),
          left: Val::Percent(50.),
          ..default()
        },
        margin: UiRect {
          left: Val::Px(-280.),
          ..default()
        },
        padding: UiRect::all(Val::Px(18.)),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::SpaceBetween,
        ..default()
      },
      background_color: Color::rgba(0.05, 0.05, 0.08, 0.92).into(),
      z_index: ZIndex::Global(40),
      ..default()
    })
    .insert(TutorialHintRoot)
    .insert(Name::new("TutorialHint"))
    .with_children(|commands| {
      // Header row: step counter (left) + Skip button (right).
      commands
        .spawn(NodeBundle {
          style: Style {
            size: Size::new(Val::Percent(100.), Val::Auto),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
          },
          ..default()
        })
        .with_children(|commands| {
          commands.spawn((
            TextBundle::from_section(
              "",
              TextStyle {
                font: assets.font.clone(),
                font_size: 18.,
                color: Color::rgb(0.55, 0.75, 0.9),
              },
            ),
            TutorialStepCounter,
          ));

          commands
            .spawn(ButtonBundle {
              style: Style {
                size: Size::new(Val::Px(74.), Val::Px(32.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
              },
              background_color: Color::rgb(0.4, 0.15, 0.15).into(),
              ..default()
            })
            .insert(TutorialSkipButton)
            .with_children(|commands| {
              commands.spawn(TextBundle::from_section(
                "Skip",
                TextStyle {
                  font: assets.font.clone(),
                  font_size: 18.,
                  color: Color::WHITE,
                },
              ));
            });
        });

      // Body: the step explanation (centred in the remaining space).
      commands.spawn((
        TextBundle::from_section(
          "",
          TextStyle {
            font: assets.font.clone(),
            font_size: 23.,
            color: Color::WHITE,
          },
        ),
        TutorialHintText,
      ));

      // Prompt: how to advance (fixed at the bottom).
      commands.spawn((
        TextBundle::from_section(
          "",
          TextStyle {
            font: assets.font.clone(),
            font_size: 19.,
            color: Color::rgb(1.0, 0.85, 0.2),
          },
        ),
        TutorialPrompt,
      ));
    });
}
