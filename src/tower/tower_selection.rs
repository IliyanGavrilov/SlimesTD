use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

use crate::assets::*;
use crate::tower::*;
use crate::{GameData, GameState, MainCamera, Player};

pub struct TowerSelectionPlugin;

impl Plugin for TowerSelectionPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems((mouse_click, tower_ui_interaction).in_set(OnUpdate(GameState::Gameplay)));
  }
}

#[derive(Component)]
pub struct TowerUpgradeUI;

fn mouse_click(
  mut commands: Commands,
  assets: Res<GameAssets>,
  windows: Query<&Window>,
  node_query: Query<(&Node, &GlobalTransform, &Visibility), With<TowerUI>>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mouse: Res<Input<MouseButton>>,
  mut clicked_tower: Query<Entity, With<TowerUpgradeUI>>,
  mut towers: Query<(Entity, &Tower, &TowerType, &Transform)>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  query: Query<Entity, With<SpriteFollower>>,
) {
  // If player isn't placing a tower
  if query.is_empty() {
    let window = windows.get_single().unwrap();
    let (camera, camera_transform) = camera_query.single();

    if mouse.just_pressed(MouseButton::Left) {
      mouse_click_interaction(
        &mut commands,
        &assets,
        &node_query,
        window,
        camera,
        camera_transform,
        &mut meshes,
        &mut materials,
        &mut clicked_tower,
        &mut towers,
      );
    }
  }
}

fn mouse_click_interaction(
  commands: &mut Commands,
  assets: &GameAssets,
  node_query: &Query<(&Node, &GlobalTransform, &Visibility), With<TowerUI>>,
  window: &Window,
  camera: &Camera,
  camera_transform: &GlobalTransform,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<ColorMaterial>>,
  clicked_tower: &mut Query<Entity, With<TowerUpgradeUI>>,
  towers: &mut Query<(Entity, &Tower, &TowerType, &Transform)>,
) {
  if let Some(position) = window.cursor_position() {
    let mouse_click_pos = window_to_world_pos(window, position, camera, camera_transform);

    if !clicked_tower.is_empty() && !cursor_above_ui(window, node_query) {
      for entity in clicked_tower.iter() {
        commands.entity(entity).despawn_recursive();
      }
    }

    for (tower_entity, tower, tower_type, transform) in towers.iter() {
      if Vec3::distance(mouse_click_pos, transform.translation) <= 25.
        && !cursor_above_ui(window, node_query)
      {
        commands.entity(tower_entity).with_children(|commands| {
          commands
            .spawn(spawn_tower_range(meshes, materials, tower.range))
            .insert(Name::new("Tower Range"))
            .insert(TowerUpgradeUI);
        });

        spawn_tower_ui(commands, assets, tower, *tower_type, transform.translation);
      }
    }
  }
}

fn tower_ui_interaction(
  //assets: Res<GameAssets>,
  mut commands: Commands,
  mut towers: Query<(Entity, &mut Tower, &TowerType, &Children)>,
  clicked_tower: Query<Entity, With<TowerUpgradeUI>>,
  keys: Res<Input<KeyCode>>,
  mut player: Query<&mut Player>,
  game_data: Res<GameData>,
  upgrades: Res<Assets<Upgrades>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut tower_range_radius: Query<&mut Mesh2dHandle>,
  // UI Buttons
  //mut images: Query<(&mut UiImage, With<SellButton>)>,
  prev_target_button_interaction: Query<
    &Interaction,
    (
      Changed<Interaction>,
      With<Button>,
      With<PreviousTargetingPriorityButton>,
    ),
  >,
  next_target_button_interaction: Query<
    &Interaction,
    (
      Changed<Interaction>,
      With<Button>,
      With<NextTargetingPriorityButton>,
    ),
  >,
  sell_button_interaction: Query<
    &Interaction,
    (Changed<Interaction>, With<Button>, With<SellButton>),
  >,
  upgrade_button_interaction: Query<
    (&Interaction, &TowerUpgradeButton),
    (Changed<Interaction>, With<Button>),
  >,
) {
  let Some(upgrades) = upgrades.get(&game_data.tower_upgrades)
    else { return; };

  if !clicked_tower.is_empty() {
    let mut player = player.single_mut();

    // Keyboard shortcuts
    for (entity, mut tower, tower_type, children) in towers.iter_mut() {
      for _ in clicked_tower.iter_many(children) {
        let mut upgrade_path_index: Option<usize> = None;

        // Sell tower
        if keys.just_pressed(KeyCode::Back) {
          // Despawn tower
          commands.entity(entity).despawn_recursive();
          // Despawn UI
          for entity in clicked_tower.iter() {
            commands.entity(entity).despawn_recursive();
          }
          player.money += (tower.total_spent / 3) as usize;
        }
        // Upgrade tower - Path 1
        else if keys.just_pressed(KeyCode::Comma) {
          upgrade_path_index = Some(0);
        }
        // Upgrade tower - Path 2
        else if keys.just_pressed(KeyCode::Period) {
          upgrade_path_index = Some(1);
        }
        // Upgrade tower - Path 3
        else if keys.just_pressed(KeyCode::Slash) {
          upgrade_path_index = Some(2);
        }
        // Change targeting priority (left)
        else if (keys.pressed(KeyCode::LControl) || keys.pressed(KeyCode::RControl))
          && keys.just_pressed(KeyCode::Tab)
        {
          tower.target.prev_target();
        }
        // Change targeting priority (right)
        else if keys.just_pressed(KeyCode::Tab) {
          tower.target.next_target();
        }

        // Upgrade
        if let Some(path_index) = upgrade_path_index {
          let i = tower.upgrades.upgrades[path_index];
          let tower_upgrades = &upgrades.upgrades[tower_type][path_index];

          if i < tower_upgrades.len() && player.money >= tower_upgrades[i].cost {
            player.money -= tower_upgrades[i].cost;
            tower.upgrade(
              &tower_upgrades[i],
              path_index,
              &mut meshes,
              &mut tower_range_radius,
            );
          }
        }

        // Button interaction

        // Targeting priority - Previous target
        for interaction in &prev_target_button_interaction {
          match interaction {
            Interaction::Clicked => {
              // Change button UI
              // for (mut image) in images.iter_mut() {
              // }

              tower.target.prev_target();
            }
            Interaction::Hovered => {
              // Change button UI !!!
            }
            Interaction::None => {
              // Change button UI !!!
            }
          }
        }

        // Targeting priority - Next target
        for interaction in &next_target_button_interaction {
          match interaction {
            Interaction::Clicked => {
              // Change button UI
              // for (mut image) in images.iter_mut() {
              // }
              tower.target.next_target();
            }
            Interaction::Hovered => {
              // Change button UI !!!
            }
            Interaction::None => {
              // Change button UI !!!
            }
          }
        }

        // Sell button
        for interaction in &sell_button_interaction {
          match interaction {
            Interaction::Clicked => {
              // Change button UI
              // for (mut image) in images.iter_mut() {
              // }

              // Despawn tower
              commands.entity(entity).despawn_recursive();
              // Despawn UI
              for entity in clicked_tower.iter() {
                commands.entity(entity).despawn_recursive();
              }
              player.money += (tower.total_spent / 3) as usize;
            }
            Interaction::Hovered => {
              // Change button UI !!!
            }
            Interaction::None => {
              // Change button UI !!!
            }
          }
        }

        // Upgrade buttons
        for (interaction, state) in &upgrade_button_interaction {
          let i = tower.upgrades.upgrades[state.path_index];
          let tower_upgrades = &upgrades.upgrades[tower_type][state.path_index];

          if i < tower_upgrades.len() && player.money >= tower_upgrades[i].cost {
            match interaction {
              Interaction::Clicked => {
                // Change button UI
                // for (mut image) in images.iter_mut() {
                // }

                player.money -= tower_upgrades[i].cost;
                tower.upgrade(
                  &tower_upgrades[i],
                  state.path_index,
                  &mut meshes,
                  &mut tower_range_radius,
                );
              }
              Interaction::Hovered => {
                // Change button UI !!!
              }
              Interaction::None => {
                // Change button UI !!!
              }
            }
          }
        }
      }
    }
  }
}
