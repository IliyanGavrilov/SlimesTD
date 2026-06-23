use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

use crate::assets::*;
use crate::tower::*;
use crate::{GameData, GameState, MainCamera, Player, game_not_paused};

pub struct TowerSelectionPlugin;

impl Plugin for TowerSelectionPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        (
          mouse_click.run_if(game_not_paused),
          tower_ui_interaction.run_if(game_not_paused),
        )
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      // Deferred despawn: removing button/image UI inline races Bevy 0.10's
      // accessibility insert and panics (B0003). `Last` runs after it, so the entity
      // is alive when the engine touches it and gone the next frame.
      .add_system(despawn_marked.in_base_set(CoreSet::Last));
  }
}

#[derive(Component)]
pub struct TowerUpgradeUI;

/// Marks an entity for deferred despawn by `despawn_marked` (see the plugin).
#[derive(Component)]
pub struct Despawning;

fn despawn_marked(mut commands: Commands, marked: Query<Entity, With<Despawning>>) {
  for entity in &marked {
    commands.entity(entity).despawn_recursive();
  }
}

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

    if cursor_above_ui(window, node_query) {
      return;
    }

    if !clicked_tower.is_empty() {
      for entity in clicked_tower.iter() {
        commands.entity(entity).insert(Despawning);
      }
    }

    // Select only the nearest tower: click targets are a little larger than the
    // sprite, so towers nudged close together would otherwise all open at once.
    let closest = towers
      .iter()
      .map(|(entity, tower, tower_type, transform)| {
        let distance = Vec3::distance(mouse_click_pos, transform.translation);
        (entity, tower, tower_type, transform, distance)
      })
      .filter(|(_, _, tower_type, _, distance)| {
        *distance <= tower_type.placement_radius() + 10.0
      })
      .min_by(|a, b| a.4.total_cmp(&b.4));

    if let Some((tower_entity, tower, tower_type, transform, _)) = closest {
      let range = spawn_tower_range(meshes, materials, tower.range, tower_type.sprite_scale());
      commands.entity(tower_entity).with_children(|commands| {
        commands
          .spawn(range)
          .insert(Name::new("Tower Range"))
          .insert(TowerUpgradeUI);
      });

      spawn_tower_ui(commands, assets, tower, *tower_type, transform.translation);
    }
  }
}

fn tower_ui_interaction(
  mut commands: Commands,
  mut towers: Query<(Entity, &mut Tower, &TowerType, &Children, Option<&mut FarmTower>)>,
  clicked_tower: Query<Entity, With<TowerUpgradeUI>>,
  keys: Res<Input<KeyCode>>,
  mut player: Query<&mut Player>,
  game_data: Res<GameData>,
  upgrades: Res<Assets<Upgrades>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut tower_range_radius: Query<&mut Mesh2dHandle>,
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

    for (entity, mut tower, tower_type, children, mut farm_tower) in towers.iter_mut() {
      for _ in clicked_tower.iter_many(children) {
        // Keyboard shortcuts. Comma/Period/Slash buy upgrade paths 1-3; Tab cycles
        // the targeting priority (Ctrl+Tab the other way); Backspace sells.
        let mut upgrade_path_index: Option<usize> = None;

        if keys.just_pressed(KeyCode::Back) {
          sell_tower(&mut commands, entity, &clicked_tower, &mut player, &tower);
        } else if keys.just_pressed(KeyCode::Comma) {
          upgrade_path_index = Some(0);
        } else if keys.just_pressed(KeyCode::Period) {
          upgrade_path_index = Some(1);
        } else if keys.just_pressed(KeyCode::Slash) {
          upgrade_path_index = Some(2);
        } else if (keys.pressed(KeyCode::LControl) || keys.pressed(KeyCode::RControl))
          && keys.just_pressed(KeyCode::Tab)
        {
          tower.target.prev_target();
        } else if keys.just_pressed(KeyCode::Tab) {
          tower.target.next_target();
        }

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
              farm_tower.as_deref_mut(),
            );
          }
        }

        // Same actions as the keyboard shortcuts, driven by the panel buttons.
        if prev_target_button_interaction.iter().any(is_clicked) {
          tower.target.prev_target();
        }
        if next_target_button_interaction.iter().any(is_clicked) {
          tower.target.next_target();
        }
        if sell_button_interaction.iter().any(is_clicked) {
          sell_tower(&mut commands, entity, &clicked_tower, &mut player, &tower);
        }

        for (interaction, state) in &upgrade_button_interaction {
          let i = tower.upgrades.upgrades[state.path_index];
          let tower_upgrades = &upgrades.upgrades[tower_type][state.path_index];

          if matches!(interaction, Interaction::Clicked)
            && i < tower_upgrades.len()
            && player.money >= tower_upgrades[i].cost
          {
            player.money -= tower_upgrades[i].cost;
            tower.upgrade(
              &tower_upgrades[i],
              state.path_index,
              &mut meshes,
              &mut tower_range_radius,
              farm_tower.as_deref_mut(),
            );
          }
        }
      }
    }
  }
}

fn is_clicked(interaction: &Interaction) -> bool {
  matches!(interaction, Interaction::Clicked)
}

/// Marks the tower and its open panel for despawn and refunds a third of what was
/// spent on it.
fn sell_tower(
  commands: &mut Commands,
  tower_entity: Entity,
  clicked_tower: &Query<Entity, With<TowerUpgradeUI>>,
  player: &mut Player,
  tower: &Tower,
) {
  commands.entity(tower_entity).insert(Despawning);
  for ui in clicked_tower.iter() {
    commands.entity(ui).insert(Despawning);
  }
  player.money += (tower.total_spent / 3) as usize;
}

#[cfg(test)]
#[path = "tower/selection_tests.rs"]
mod tests;