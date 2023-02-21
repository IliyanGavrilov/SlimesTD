use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::{GameState, MainCamera, Player, SpriteFollower, Tower, TowerType, Upgrades, window_to_world_pos};

pub struct TowerSelectionPlugin;

impl Plugin for TowerSelectionPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(tower_click)
        .with_system(tower_ui_interaction));
  }
}

#[derive(Component)]
pub struct TowerUpgradeUI;

fn tower_click(
  mut commands: Commands,
  windows: Res<Windows>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mouse: Res<Input<MouseButton>>,
  mut clicked_tower: Query<Entity, (With<Handle<ColorMaterial>>, With<TowerUpgradeUI>)>,
  mut towers: Query<(Entity, &Tower, &TowerType, &Transform)>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  query: Query<Entity, With<SpriteFollower>>
) {
  // If player isn't placing a tower
  if query.is_empty() {
    let window = windows.get_primary().unwrap();
    let (camera, camera_transform) = camera_query.single();
  
    if mouse.just_pressed(MouseButton::Right) {
      mouse_click_interaction(&mut commands, window, camera, camera_transform, &mut meshes,
                              &mut materials, &mut clicked_tower, &mut towers);
    }
  }
}

fn mouse_click_interaction(
  commands: &mut Commands,
  window: &Window,
  camera: &Camera,
  camera_transform: &GlobalTransform,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<ColorMaterial>>,
  clicked_tower: &mut Query<Entity, (With<Handle<ColorMaterial>>, With<TowerUpgradeUI>)>,
  towers: &mut Query<(Entity, &Tower, &TowerType, &Transform)>
) {
  if let Some(position) = window.cursor_position() {
    let mouse_click_pos =
      window_to_world_pos(window, position, camera, camera_transform);
  
    if !clicked_tower.is_empty() {
      let entity = clicked_tower.single_mut();
      commands.entity(entity).despawn_recursive();
    }
    
    for (tower_entity,
      tower,
      _,
      transform) in towers.iter() {
      if Vec3::distance(mouse_click_pos, transform.translation) <= 25. {
        commands.entity(tower_entity)
          .with_children(|commands| {
            commands.spawn(MaterialMesh2dBundle {
              mesh: meshes.add(shape::Circle::new(tower.range as f32).into())
                .into(),
              material: materials.add(ColorMaterial::from(
                Color::rgba_u8(0, 0, 0, 85))),
              transform: Transform::from_translation(Vec3::new(0., 0., -0.5)),
              ..default()
            }).insert(TowerUpgradeUI);
          });
      }
    }
  }
}

fn tower_ui_interaction (
  mut commands: Commands,
  mut towers: Query<(Entity, &mut Tower, &TowerType)>,
  mut clicked_tower: Query<Entity, (With<Handle<ColorMaterial>>, With<TowerUpgradeUI>)>,
  keys: Res<Input<KeyCode>>,
  mut player: Query<&mut Player>,
  upgrades: Res<Upgrades>
) {
  if !clicked_tower.is_empty() {
    let mut player = player.single_mut();
  
    for (entity, mut tower, tower_type) in towers.iter_mut() {
      let mut upgrade_path_index: Option<usize> = None;
    
      // Sell tower
      if keys.just_pressed(KeyCode::Back) {
        commands.entity(entity).despawn_recursive();
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
        && keys.just_pressed(KeyCode::Tab) {
        tower.target.prev_target();
      }
      // Change targeting priority (right)
      else if keys.just_pressed(KeyCode::Tab) {
        tower.target.next_target();
      }
    
      if let Some(path_index) = upgrade_path_index {
        let i = tower.upgrades.upgrades[path_index];
        let tower_upgrades = &upgrades.upgrades[tower_type][path_index];
      
        if i < tower_upgrades.len() && player.money >= tower_upgrades[i].cost {
          player.money -= tower_upgrades[i].cost;
          tower.upgrade(&tower_upgrades[i], path_index);
        }
      }
    }
  }
}