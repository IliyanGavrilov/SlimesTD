use bevy::prelude::*;

pub use crate::{Bullet, Movement, tower_type::TowerType, GameAssets, targeting_priority::*};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Tower>()
       .add_system(tower_shooting);
  }
}

#[derive(Bundle)]
pub struct TowerBundle {
  pub tower_type: TowerType,
  pub tower: Tower,
  pub sprite: SpriteBundle,
  pub name: Name
}

impl Default for TowerBundle {
  fn default() -> Self {
    TowerBundle {
      tower_type: TowerType::Nature,
      tower: Tower::new(
        Vec3::new(20., 0., 0.),
        1,
        Timer::from_seconds(1., TimerMode::Repeating),
        10,
        100
      ),
      sprite: Default::default(),
      name: Name::new("NatureTower")
    }
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

impl Tower {
  pub fn new(
    bullet_spawn_offset: Vec3,
    damage: i32,
    attack_speed: Timer,
    range: i32,
    price: i32,
  ) -> Self {
    Self {
      bullet_spawn_offset,
      damage,
      attack_speed,
      range,
      price,
      sell_price: (price/3) as i32,
      target: TargetingPriority::CLOSE
      // !!! ..default()
    }
  }
}

pub fn spawn_tower(
  commands: &mut Commands,
  tower_type: TowerType,
  assets: &GameAssets,
  position: Vec3
) {
  commands.spawn(tower_type.get_tower(assets, position));
}

fn tower_shooting(
  mut commands: Commands,
  assets: Res<GameAssets>, // Bullet assets
  mut towers: Query<(Entity, &mut Tower, &TowerType, &mut Transform, &GlobalTransform)>,
  enemies: Query<&GlobalTransform, With<Enemy>>, // Gets all entities With the Enemy component
  time: Res<Time>,
) {
  for (tower_entity, mut tower, tower_type, mut tower_transform, transform) in &mut towers {
    tower.attack_speed.tick(time.delta());
    
    let bullet_spawn_pos = transform.translation() + tower.bullet_spawn_offset;
    
    // If the attack cooldown finished, spawn bullet
    if tower.attack_speed.just_finished() {
      let direction = match &tower.target {
        TargetingPriority::FIRST => first_enemy_direction(&enemies, bullet_spawn_pos),
        TargetingPriority::LAST => last_enemy_direction(&enemies, bullet_spawn_pos),
        TargetingPriority::CLOSE => closest_enemy_direction(&enemies, bullet_spawn_pos),
        TargetingPriority::STRONGEST => strongest_enemy_direction(&enemies, bullet_spawn_pos),
        TargetingPriority::WEAKEST => weakest_enemy_direction(&enemies, bullet_spawn_pos)
      };
      
      // If there is an enemy in the tower's range!!! (if direction != None), then shoot bullet
      if let Some(direction) = direction {
        // Calculate angle between tower and enemy
        let mut angle = direction.angle_between(transform.translation());
        if transform.translation().y > direction.y { // flip angle if enemy is below tower
          angle = -angle;
        }
  
        let bullet_transform = Transform::from_translation(tower.bullet_spawn_offset);
        
        // Rotate tower to face enemy it is attacking, based on enemy's location
        tower_transform.rotation = Quat::from_rotation_z(angle);
        
        let (bullet, bullet_movement, bullet_asset) = tower_type.get_bullet(tower.damage, &assets);
        
        // Make bullet a child of tower
        commands.entity(tower_entity).with_children(|commands| {
          commands.spawn(SpriteBundle {
            texture: bullet_asset,
            transform: bullet_transform,
            sprite: Sprite { // !!! Temp
              custom_size: Some(Vec2::new(40., 22.)),
              ..default()
            },
            ..default()
          })
            .insert(bullet)
            .insert(bullet_movement)
            .insert(Name::new("Bullet"));
        });
      }
    }
  }
}