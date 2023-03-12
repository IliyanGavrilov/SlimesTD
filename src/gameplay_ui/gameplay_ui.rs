use bevy::prelude::*;

use crate::assets::*;
use crate::gameplay_ui::*;
use crate::GameState;

#[derive(Component)]
pub struct GameplayUIRoot;

#[derive(Component)]
pub struct HealthUI;

#[derive(Component)]
pub struct MoneyUI;

pub struct GameplayUIPlugin;

impl Plugin for GameplayUIPlugin {
  fn build(&self, app: &mut App) {
    app.add_system_set(
        SystemSet::on_enter(GameState::Gameplay)
          .with_system(spawn_gameplay_ui),
      )
      .add_system_set(
        SystemSet::on_update(GameState::Gameplay)
          .with_system(update_gameplay_ui),
      );
  }
}

fn update_gameplay_ui(
  player: Query<&Player>,
  base: Query<&Base>,
  mut money_ui: Query<&mut Text, (With<MoneyUI>, Without<HealthUI>)>,
  mut health_ui: Query<&mut Text, With<HealthUI>>,
) {
  let player = player.single();
  let base = base.single();
  let mut money = money_ui.single_mut();
  let mut health = health_ui.single_mut();
  
  *money = Text::from_section(
    format!("{}", player.money),
    money.sections[0].style.clone(),
  );
  *health = Text::from_section(
    format!("{}", base.health),
    health.sections[0].style.clone(),
  );
}

fn spawn_gameplay_ui(
  mut commands: Commands,
  assets: Res<GameAssets>) {
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
        }).insert(GameplayUIRoot)
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
                  font_size: 36.0,
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
                  font_size: 36.0,
                  color: Color::WHITE,
                },
              ),
              ..default()
            })
            .insert(MoneyUI)
            .insert(Name::new("Money"));
        });
    })
    .insert(Name::new("MoneyAndHealthUI"));
}