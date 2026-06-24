use bevy::prelude::*;

use crate::assets::*;
use crate::map::*;
use crate::{GameData, GameState};

pub struct MapSelectionPlugin;

impl Plugin for MapSelectionPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<GameDifficulty>()
      .add_system(init_selected_map.in_schedule(OnExit(GameState::AssetLoading)))
      .add_system(spawn_map_selection.in_schedule(OnEnter(GameState::MapSelection)))
      .add_systems(
        (select_button_clicked, back_button_clicked).in_set(OnUpdate(GameState::MapSelection)),
      )
      .add_system(cleanup_map_selection.in_schedule(OnExit(GameState::MapSelection)));
  }
}

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameDifficulty {
  #[default]
  Normal,
  /// Reduced game speed = 5000 starting gold, all enemies have 1 HP.
  Test,
}

#[derive(Resource)]
pub struct SelectedMap(pub Handle<Map>);

#[derive(Component)]
struct MapSelectionRoot;

#[derive(Component)]
struct SelectMapButton(Handle<Map>, GameDifficulty);

#[derive(Component)]
struct BackButton;

fn init_selected_map(mut commands: Commands, game_data: Res<GameData>) {
  commands.insert_resource(SelectedMap(game_data.map.clone()));
}

fn select_button_clicked(
  mut commands: Commands,
  interactions: Query<(&Interaction, &SelectMapButton), Changed<Interaction>>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  for (interaction, button) in &interactions {
    if matches!(interaction, Interaction::Clicked) {
      commands.insert_resource(SelectedMap(button.0.clone()));
      commands.insert_resource(button.1);
      next_state.set(GameState::Gameplay);
    }
  }
}

fn back_button_clicked(
  interactions: Query<&Interaction, (With<BackButton>, Changed<Interaction>)>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  for interaction in &interactions {
    if matches!(interaction, Interaction::Clicked) {
      next_state.set(GameState::MainMenu);
    }
  }
}

fn cleanup_map_selection(mut commands: Commands, roots: Query<Entity, With<MapSelectionRoot>>) {
  for entity in &roots {
    commands.entity(entity).despawn_recursive();
  }
}

fn spawn_map_selection(
  mut commands: Commands,
  assets: Res<GameAssets>,
  game_data: Res<GameData>,
  maps: Res<Assets<Map>>,
) {
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
    .insert(MapSelectionRoot)
    .insert(Name::new("MapSelectionScreen"))
    .with_children(|commands| {
      commands.spawn(TextBundle {
        style: Style {
          margin: UiRect::bottom(Val::Percent(4.)),
          ..default()
        },
        text: Text::from_section(
          "Select Map",
          TextStyle {
            font: assets.font.clone(),
            font_size: 70.,
            color: Color::CYAN,
          },
        ),
        ..default()
      });

      commands
        .spawn(NodeBundle {
          style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            ..default()
          },
          ..default()
        })
        .with_children(|commands| {
          spawn_map_card(commands, &assets, &maps, game_data.map.clone(), "Level 1");
          spawn_map_card(commands, &assets, &maps, game_data.map2.clone(), "Level 2");
          spawn_map_card(commands, &assets, &maps, game_data.map3.clone(), "Level 3");
        });

      commands
        .spawn(ButtonBundle {
          style: Style {
            size: Size::new(Val::Px(220.), Val::Px(65.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::top(Val::Percent(4.)),
            ..default()
          },
          background_color: Color::DARK_GRAY.into(),
          ..default()
        })
        .insert(BackButton)
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

fn spawn_map_card(
  commands: &mut ChildBuilder,
  assets: &GameAssets,
  maps: &Assets<Map>,
  map_handle: Handle<Map>,
  name: &str,
) {
  commands
    .spawn(NodeBundle {
      style: Style {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(16.)),
        margin: UiRect::all(Val::Px(24.)),
        size: Size::new(Val::Px(240.), Val::Auto),
        ..default()
      },
      background_color: Color::rgb(0.2, 0.2, 0.2).into(),
      ..default()
    })
    .with_children(|commands| {
      commands.spawn(TextBundle {
        style: Style {
          margin: UiRect::bottom(Val::Px(12.)),
          ..default()
        },
        text: Text::from_section(
          name,
          TextStyle {
            font: assets.font.clone(),
            font_size: 32.,
            color: Color::WHITE,
          },
        ),
        ..default()
      });

      if let Some(map) = maps.get(&map_handle) {
        spawn_minimap(commands, map);
      }

      commands
        .spawn(NodeBundle {
          style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            margin: UiRect::top(Val::Px(14.)),
            ..default()
          },
          ..default()
        })
        .with_children(|commands| {
          spawn_difficulty_button(
            commands,
            assets,
            map_handle.clone(),
            GameDifficulty::Normal,
            "Normal",
            Color::rgb(0.25, 0.55, 0.25),
          );
          spawn_difficulty_button(
            commands,
            assets,
            map_handle,
            GameDifficulty::Test,
            "Test",
            Color::rgb(0.55, 0.35, 0.10),
          );
        });
    });
}

fn spawn_difficulty_button(
  commands: &mut ChildBuilder,
  assets: &GameAssets,
  map_handle: Handle<Map>,
  difficulty: GameDifficulty,
  label: &str,
  color: Color,
) {
  commands
    .spawn(ButtonBundle {
      style: Style {
        size: Size::new(Val::Px(100.), Val::Px(46.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::horizontal(Val::Px(4.)),
        ..default()
      },
      background_color: color.into(),
      ..default()
    })
    .insert(SelectMapButton(map_handle, difficulty))
    .with_children(|commands| {
      commands.spawn(TextBundle {
        text: Text::from_section(
          label,
          TextStyle {
            font: assets.font.clone(),
            font_size: 22.,
            color: Color::WHITE,
          },
        ),
        ..default()
      });
    });
}

fn spawn_minimap(commands: &mut ChildBuilder, map: &Map) {
  const TILE_PX: f32 = 10.;

  // RON row 0 is the top of the map. After load_map runs, tiles are reversed
  // (Bevy Y-up). Display them top-down regardless of whether they've been flipped.
  let rows: Vec<&Vec<Tile>> = if map.initialized {
    map.tiles.iter().rev().collect()
  } else {
    map.tiles.iter().collect()
  };

  commands
    .spawn(NodeBundle {
      style: Style {
        flex_direction: FlexDirection::Column,
        ..default()
      },
      ..default()
    })
    .with_children(|commands| {
      for row in rows {
        commands
          .spawn(NodeBundle {
            style: Style {
              flex_direction: FlexDirection::Row,
              ..default()
            },
            ..default()
          })
          .with_children(|commands| {
            for tile in row {
              commands.spawn(NodeBundle {
                style: Style {
                  size: Size::new(Val::Px(TILE_PX), Val::Px(TILE_PX)),
                  ..default()
                },
                background_color: tile_color(tile).into(),
                ..default()
              });
            }
          });
      }
    });
}

fn tile_color(tile: &Tile) -> Color {
  match tile {
    Tile::Grass => Color::rgb(0.35, 0.55, 0.25),
    Tile::Water => Color::rgb(0.25, 0.45, 0.75),
    Tile::Path(_) | Tile::Lane(..) => Color::rgb(0.65, 0.55, 0.35),
    Tile::Spawn => Color::rgb(0.2, 0.9, 0.2),
    Tile::End => Color::rgb(0.9, 0.2, 0.2),
    Tile::Empty => Color::rgb(0.1, 0.1, 0.1),
  }
}
