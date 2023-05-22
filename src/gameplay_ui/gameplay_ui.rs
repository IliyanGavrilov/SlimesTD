use bevy::prelude::*;

use crate::assets::*;
use crate::gameplay_ui::*;
use crate::{GameData, GameState, Waves};

#[derive(Component)]
pub struct GameplayUIRoot;

#[derive(Component)]
pub struct HealthUI;

#[derive(Component)]
pub struct MoneyUI;

#[derive(Component)]
pub struct RoundUI;

pub struct GameplayUIPlugin;

impl Plugin for GameplayUIPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(spawn_gameplay_ui.in_schedule(OnEnter(GameState::Gameplay)))
      .add_system(update_gameplay_ui.in_set(OnUpdate(GameState::Gameplay)));
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
  let Some(waves) = waves.get(&game_data.enemy_waves)
    else { return; };
  let mut money = money_ui.single_mut();
  let mut health = health_ui.single_mut();
  let mut round = round_ui.single_mut();

  *money = Text::from_section(format!("{}", player.money), money.sections[0].style.clone());
  *health = Text::from_section(format!("{}", base.health), health.sections[0].style.clone());

  if waves.current + 1 <= waves.waves.len() {
    *round = Text::from_section(
      format!("{}/{}", waves.current + 1, waves.waves.len()),
      round.sections[0].style.clone(),
    );
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
                position_type: PositionType::Absolute,
                margin: UiRect {
                  left: Val::Percent(92.5),
                  right: Val::Percent(4.5),
                  top: Val::Percent(1.5),
                  bottom: Val::Percent(2.5),
                },
                ..default()
              },
              text: Text::from_section(
                "",
                TextStyle {
                  font: assets.font.clone(),
                  font_size: 50.,
                  color: Color::WHITE,
                },
              ),
              ..default()
            })
            .insert(RoundUI)
            .insert(Name::new("Round"));
        });
    })
    .insert(Name::new("GameplayUI"));
}
