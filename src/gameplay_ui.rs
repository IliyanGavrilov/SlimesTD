use bevy::prelude::*;

use crate::{Base, GameState, Player};

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
  // The without here prevents queries from potentially matching the same text component
  mut money_ui: Query<&mut Text, (With<MoneyUI>, Without<HealthUI>)>,
  mut health_ui: Query<&mut Text, With<HealthUI>>,
) {
  let player = player.single();
  let base = base.single();
  let mut money = money_ui.single_mut();
  let mut health = health_ui.single_mut();
  
  *money = Text::from_section(
    format!("Money: {}", player.money),
    money.sections[0].style.clone(),
  );
  *health = Text::from_section(
    format!("Health: {}", base.health),
    health.sections[0].style.clone(),
  );
}

fn spawn_gameplay_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        position_type: PositionType::Absolute,
        justify_content: JustifyContent::FlexStart,
        flex_direction: FlexDirection::Column,
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
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexStart,
            align_self: AlignSelf::FlexStart,
            flex_direction: FlexDirection::Row,
            ..default()
          },
          ..default()
        })
        .with_children(|commands| {
          commands
            .spawn(TextBundle {
              style: Style {
                margin: UiRect::all(Val::Percent(1.2)),
                ..default()
              },
              text: Text::from_section(
                "Player Money: XX",
                TextStyle {
                  font: asset_server.load("font/FiraSans-Bold.ttf"),
                  font_size: 36.0,
                  color: Color::WHITE,
                },
              ),
              ..default()
            })
            .insert(MoneyUI);
          commands
            .spawn(TextBundle {
              style: Style {
                margin: UiRect::all(Val::Percent(1.2)),
                ..default()
              },
              text: Text::from_section(
                "Player Health: XX",
                TextStyle {
                  font: asset_server.load("font/FiraSans-Bold.ttf"),
                  font_size: 36.0,
                  color: Color::WHITE,
                },
              ),
              ..default()
            })
            .insert(HealthUI);
        });
    });
}