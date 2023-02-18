use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::{GameState, MainCamera, Player, TargetingPriority, Tower, TowerType, window_to_world_pos};

pub struct TowerUpgradePlugin;

impl Plugin for TowerUpgradePlugin {
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
  mut clicked_tower: Query<(Entity, &Tower, &TowerType, &Transform), With<TowerUpgradeUI>>,
  mut towers: Query<(Entity, &Tower, &TowerType, &Transform)>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>
) {
  let window = windows.get_primary().unwrap();
  let (camera, camera_transform) = camera_query.single();
  
  if mouse.just_pressed(MouseButton::Left) {
    mouse_click_interaction(&mut commands, window, camera, camera_transform, &mut meshes,
                            &mut materials, &mut clicked_tower, &mut towers);
  }
}

fn mouse_click_interaction(
  commands: &mut Commands,
  window: &Window,
  camera: &Camera,
  camera_transform: &GlobalTransform,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<ColorMaterial>>,
  clicked_tower: &mut Query<(Entity, &Tower, &TowerType, &Transform), With<TowerUpgradeUI>>,
  towers: &mut Query<(Entity, &Tower, &TowerType, &Transform)>,
) {
  if let Some(position) = window.cursor_position() {
    let mouse_click_pos =
      window_to_world_pos(window, position, camera, camera_transform);
  
    if !clicked_tower.is_empty() {
      let (entity, ..) = clicked_tower.single_mut();
      commands.entity(entity).remove::<(Handle<ColorMaterial>, TowerUpgradeUI)>();
    }
    
    for (tower_entity,
      _,
      tower_type,
      transform) in towers.iter() {
      if Vec3::distance(mouse_click_pos, transform.translation) <= 25. {
        
        commands.entity(tower_entity)
          .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(tower_type.get_range() as f32).into())
              .into(),
            material: materials.add(ColorMaterial::from(
              Color::rgba_u8(0, 0, 0, 85))),
            transform: *transform,
            ..default()
          }).insert(TowerUpgradeUI);
      }
    }
  }
}

fn tower_ui_interaction (
  mut commands: Commands,
  mut towers: Query<(Entity, &mut Tower, &TowerType), With<TowerUpgradeUI>>,
  keys: Res<Input<KeyCode>>,
  mut player: Query<&mut Player>
) {
  let mut player = player.single_mut();

  for (entity, mut tower, tower_type) in towers.iter_mut() {
    // Sell tower
    if keys.just_pressed(KeyCode::Back) {
      commands.entity(entity).despawn_recursive();
      player.money += (tower_type.get_price()/3) as usize;
    }
    // Upgrade tower
    else if keys.just_pressed(KeyCode::Comma) {
      //player.money -= tower.;
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
  }
}