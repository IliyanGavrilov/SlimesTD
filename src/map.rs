use bevy::prelude::*;
use crate::{Base, Enemy, GameState, Movement, Path};

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
      Vec3::new(-475., -450., 0.), // Spawn
      Vec3::new(-475., -431.2, 0.),
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
      Vec3::new(510., -450., 0.) // Despawn
    ]
  })
}

fn update_enemy_checkpoint(
  mut commands: Commands,
  mut enemies: Query<(Entity, &Enemy, &mut Movement, &Transform, &mut Path)>,
  map: Res<MapPath>,
  mut base: Query<&mut Base>
) {
  let mut base = base.single_mut();
  
  for (entity,
    enemy,
    mut movement,
    transform,
    mut path) in &mut enemies {
    if path.index >= map.checkpoints.len() - 1 {
      damage_base(&mut commands, &entity, enemy.health, &mut base);
    }
    else if Vec3::distance(transform.translation, map.checkpoints[path.index]) <= 10. {
      path.index += 1;
      movement.direction = Vec3::new(0., 0., 0.,);
      movement.direction = map.checkpoints[path.index] - transform.translation;
    }
  }
}

fn damage_base(
  commands: &mut Commands,
  entity: &Entity,
  enemy_health: u32,
  base: &mut Base
) {
  commands.entity(*entity).despawn_recursive();
  
  if base.health > 0 {
    base.health -= enemy_health as i32;
  }
  
  if base.health <= 0{
    base.health = 0;
    info!("GAME OVER");
  }
}