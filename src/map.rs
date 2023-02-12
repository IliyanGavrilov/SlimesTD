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
      Vec3::new(500., 0., 0.),
      Vec3::new(500., 500., 0.),
      Vec3::new(1000., 500., 0.),
      Vec3::new(1000., 700., 0.),
      Vec3::new(1200., 700., 0.),
      Vec3::new(1200., 600., 0.),
      Vec3::new(1400., 600., 0.),
      Vec3::new(1400., 400., 0.),
      Vec3::new(1200., 400., 0.),
      Vec3::new(1200., 300., 0.),
      Vec3::new(1000., 300., 0.),
      Vec3::new(1000., 150., 0.),
      Vec3::new(1200., 150., 0.),
      Vec3::new(1200., 0., 0.)
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
    else if Vec3::distance(transform.translation, map.checkpoints[path.index]) <= 5. {
      path.index += 1;
      movement.direction = Vec3::new(0., 0., 0.,);
      movement.direction = map.checkpoints[path.index] - transform.translation;
    }
  }
}

fn damage_base(
  commands: &mut Commands,
  entity: &Entity,
  enemy_health: i32,
  base: &mut Base
) {
  commands.entity(*entity).despawn_recursive();
  
  if base.health > 0 {
    base.health -= enemy_health;
  }
  
  if base.health <= 0{
    info!("GAME OVER");
  }
}