use bevy::prelude::*;

pub use crate::targeting_priority::*;
pub use crate::GameAssets;
pub use crate::bullet::Bullet;
pub use crate::movement::Movement;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Tower>()
       .add_system(tower_shooting);
  }
}

//#[derive(Component)] // !!!Debugging
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
  pub bullet_spawn_offset: Vec3,
  pub damage: i32,
  pub attack_speed: Timer,
  pub range: i32,
  pub price: i32,
  pub sell_price: i32,
  pub target: TargetingPriority
}

fn tower_shooting(
  mut commands: Commands,
  assets: Res<GameAssets>, // Bullet assets
  mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
  enemies: Query<&GlobalTransform, With<Enemy>>, // Gets all entities With the Enemy component
  time: Res<Time>,
) {
  for (tower_entity, mut tower, transform) in &mut towers {
    tower.attack_speed.tick(time.delta());
    
    let bullet_spawn_pos = transform.translation() + tower.bullet_spawn_offset;
    
    // If the attack cooldown finished, spawn bullet
    if tower.attack_speed.finished() {
      let direction = match &tower.target {
        TargetingPriority::FIRST => first_enemy_direction(&enemies, bullet_spawn_pos),
        TargetingPriority::LAST => last_enemy_direction(&enemies, bullet_spawn_pos),
        TargetingPriority::CLOSE => closest_enemy_direction(&enemies, bullet_spawn_pos),
        TargetingPriority::STRONGEST => strongest_enemy_direction(&enemies, bullet_spawn_pos),
        TargetingPriority::WEAKEST => weakest_enemy_direction(&enemies, bullet_spawn_pos)
      };
      
      // If there is an enemy in the tower's range!!! (if direction != None), then shoot bullet
      if let Some(direction) = direction {
        // Rotate bullet, based on enemy location
        let mut angle = direction.angle_between(tower.bullet_spawn_offset);
        if transform.translation().y > direction.y { // flip angle if enemy is below tower
          angle = -angle;
        }
        let bullet_transform = Transform::from_translation(tower.bullet_spawn_offset);
        
        // Make bullet a child of tower
        commands.entity(tower_entity).with_children(|commands| {
          commands.spawn(SpriteBundle {
            texture: assets.bullet.clone(),
            transform: bullet_transform.with_rotation(Quat::from_rotation_z(angle)),
            sprite: Sprite {
              custom_size: Some(Vec2::new(40., 22.)),
              ..default()
            },
            ..default()
          })
            .insert(Bullet {
              damage: tower.damage,
              lifetime: Timer::from_seconds(5., TimerMode::Once)
            })
            .insert(Movement {
              direction,
              speed: 750.,
            })
            .insert(Name::new("Bullet"));
        });
      }
    }
  }
}