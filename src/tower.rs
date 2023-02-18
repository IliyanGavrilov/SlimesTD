use std::time::Duration;
use bevy::prelude::*;

pub use crate::{GameState, Bullet, Movement, tower_type::TowerType, GameAssets,
                targeting_priority::{*, TargetingPriority::*}};
use crate::{TowerStat, TowerUpgrades, Upgrade};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Tower>()
      .add_system_set(SystemSet::on_update(GameState::Gameplay)
        .with_system(tower_shooting));
  }
}

#[derive(Bundle)]
pub struct TowerBundle {
  pub tower_type: TowerType,
  pub tower: Tower,
  pub sprite: SpriteBundle,
  pub name: Name
}

//#[derive(Component)] // !!!Debugging
#[derive(Reflect, Clone, Component, Default)]
#[reflect(Component)]
pub struct Tower {
  pub bullet_spawn_offset: Vec3,
  pub damage: u32,
  pub attack_speed: f32,
  pub range: u32,
  pub price: u32,
  pub sell_price: u32,
  pub upgrades: TowerUpgrades,
  pub target: TargetingPriority,
  pub shooting_timer: Timer,
  pub total_spent: u32,
  // Flag to stop timer from counting when there are no enemies
  pub first_enemy_appeared: bool
}

impl Tower {
  pub fn new(
    bullet_spawn_offset: Vec3,
    damage: u32,
    attack_speed: f32,
    range: u32,
    price: u32,
  ) -> Self {
    Self {
      bullet_spawn_offset,
      damage,
      attack_speed,
      range,
      price,
      total_spent: price,
      sell_price: (price/3) as u32,
      first_enemy_appeared: false,
      shooting_timer: Timer::new(Duration::from_millis((1000. * attack_speed) as u64),
                                 TimerMode::Repeating),
      ..default()
    }
  }
  
  pub fn upgrade(&mut self, upgrade: &Upgrade, path_index: usize) {
    self.total_spent += upgrade.cost as u32;
    for (k, v) in &upgrade.upgrade {
      match *k {
        TowerStat::Damage => {self.damage += *v as u32}
        TowerStat::AttackSpeed => {
          self.attack_speed -= (*v as f32 )* 0.01 * self.attack_speed;
          self.shooting_timer.reset();
          self.shooting_timer.set_duration(Duration::from_millis(
            (1000. * self.attack_speed) as u64));
        }
        TowerStat::Range => {self.range += *v as u32}
      }
    }
  
    self.upgrades.upgrades[path_index] += 1;
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
  enemies: Query<(&GlobalTransform, &Enemy, &Movement, &TimeAlive)>, // Gets all entities With the Enemy component
  time: Res<Time>,
) {
  for (tower_entity,
    mut tower,
    tower_type,
    mut tower_transform,
    transform) in &mut towers {
    // Check if an enemy is in range so we can tick the timer
    if enemy_in_range(&tower, &tower_transform, &enemies) {
      let bullet_spawn_pos = transform.translation() + tower.bullet_spawn_offset;
    
      let direction = match &tower.target {
        FIRST => first_enemy_direction(&enemies,
                                       bullet_spawn_pos,
                                       tower.range),
        LAST => last_enemy_direction(&enemies,
                                     bullet_spawn_pos,
                                     tower.range),
        CLOSE => closest_enemy_direction(&enemies,
                                         bullet_spawn_pos,
                                         tower.range),
        STRONG => strongest_enemy_direction(&enemies,
                                               bullet_spawn_pos,
                                               tower.range),
        WEAK => weakest_enemy_direction(&enemies,
                                           bullet_spawn_pos,
                                           tower.range)
      };
    
      // If there is an enemy in the tower's range!!! (if direction != None), then shoot bullet
      if let Some(direction) = direction {
        // If the attack cooldown finished OR if there was no enemy spawned before, spawn bullet
        if tower.shooting_timer.just_finished() || tower.first_enemy_appeared {
          tower.first_enemy_appeared = false;
        
          // Calculate angle between tower and enemy
          let mut angle = direction.angle_between(tower.bullet_spawn_offset);
          if tower.bullet_spawn_offset.y > direction.y { // flip angle if enemy is below tower
            angle = -angle;
          }
          
          // Rotate tower to face enemy it is attacking, based on enemy's location
          tower_transform.rotation = Quat::from_rotation_z(angle);
        
          // Make bullet a child of tower
          commands.entity(tower_entity).with_children(|commands| {
            commands.spawn(tower_type.get_bullet(
              tower.damage,
              &assets,
              Transform::from_translation(tower.bullet_spawn_offset)));
          });
        }
      
        tower.shooting_timer.tick(time.delta());
      }
    } else {
      tower.shooting_timer.reset();
      tower.first_enemy_appeared = true;
    }
  }
}

fn enemy_in_range(
  tower: &Mut<Tower>,
  tower_transform: &Mut<Transform>,
  enemies: &Query<(&GlobalTransform, &Enemy, &Movement, &TimeAlive)>
) -> bool {
  for (enemy_transform, ..) in enemies {
    if Vec3::distance(tower_transform.translation,
                      enemy_transform.translation()) <= tower.range as f32 {
      return true
    }
  }
  
  return false
}