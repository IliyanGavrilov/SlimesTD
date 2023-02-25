use bevy::prelude::*;

use crate::gameplay_ui::*;
use crate::{Enemy, GameState, Path};
use crate::movement::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app.add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(update_enemy_checkpoint))
      .add_startup_system_to_stage(StartupStage::PreStartup, load_map);
  }
}

#[derive(Resource)]
pub struct MapPath {
  pub checkpoints: Vec<Vec3>,
}

fn load_map(
  mut commands: Commands
) {
  commands.insert_resource(MapPath {
    checkpoints: vec![
      Vec3::new(-475., -440., 0.), // Spawn
      Vec3::new(-475., -200., 0.),
      Vec3::new(-210., -200., 0.),
      Vec3::new(-210., 15., 0.),
      Vec3::new(10., 15., 0.),
      Vec3::new(10., -300., 0.),
      Vec3::new(210., -300., 0.),
      Vec3::new(210., 310., 0.),
      Vec3::new(370., 310., 0.),
      Vec3::new(370., 25., 0.),
      Vec3::new(510., 25., 0.),
      Vec3::new(510., -450., 0.),
      Vec3::new(510., -440., 0.) // Despawn
    ]
  })
}

fn update_enemy_checkpoint(
  mut commands: Commands,
  mut enemies: Query<(Entity, &Enemy, &mut Movement, &mut Transform, &mut Path)>,
  map: Res<MapPath>,
  mut base: Query<&mut Base>,
  time: Res<Time>
) {
  let mut base = base.single_mut();
  
  for (entity,
    enemy,
    mut movement,
    mut transform,
    mut path) in &mut enemies {
    if path.index >= map.checkpoints.len() - 1 {
      damage_base(&mut commands, &entity, enemy.health, &mut base);
    }
    
    let distance = map.checkpoints[path.index] - transform.translation;
    if distance == Vec3::ZERO {
      path.index += 1;
      continue;
    }
    let enemy_movement = distance.normalize() * movement.speed * time.delta_seconds();
    
    if enemy_movement.length() > distance.length() {
      transform.translation = map.checkpoints[path.index];
      //movement.direction = Vec3::new(0., 0., 0.,);
      movement.distance_travelled += distance.length();
      movement.direction = map.checkpoints[path.index] - transform.translation;
      path.index += 1;
    }
    else {
      movement.distance_travelled += enemy_movement.length();
      transform.translation += enemy_movement;
    }
  }
}