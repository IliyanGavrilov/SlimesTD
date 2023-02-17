use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::{GameState, MainCamera, Tower, TowerType, window_to_world_pos};

pub struct TowerUpgradePlugin;

impl Plugin for TowerUpgradePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(tower_click));
  }
}

#[derive(Component)]
pub struct TowerUpgradeUI;

fn tower_click(
  mut commands: Commands,
  windows: Res<Windows>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mouse: Res<Input<MouseButton>>,
  mut towers: Query<(Entity, &mut Tower, &TowerType, &Transform)>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut query: Query<Entity, With<TowerUpgradeUI>>
) {
  let window = windows.get_primary().unwrap();
  let (camera, camera_transform) = camera_query.single();
  for (tower_entity,
    mut tower,
    tower_type,
    transform) in &mut towers {
    if mouse.just_pressed(MouseButton::Left) {
      
      if !query.is_empty() {
        for entity in &mut query {
          commands.entity(entity).despawn_recursive();
        }
      }
      
      if let Some(position) = window.cursor_position() {
        let mouse_click_pos =
          window_to_world_pos(window, position, camera, camera_transform);
        
        if Vec3::distance(mouse_click_pos, transform.translation) <= 25. {
          commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(tower_type.get_range() as f32).into())
              .into(),
            material: materials.add(ColorMaterial::from(
              Color::rgba_u8(0, 0, 0, 85))),
            transform: *transform,
            ..default()
          })
            .insert(TowerUpgradeUI);
        }
      }
    }
  }
}