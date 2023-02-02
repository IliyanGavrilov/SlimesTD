use bevy::prelude::*;
use  bevy_inspector_egui::Inspectable;
use strum_macros::{EnumIter, Display};
use strum::IntoEnumIterator;

pub use crate::targeting_priority::*;
pub use crate::GameAssets;
pub use crate::bullet::Bullet;
pub use crate::movement::Movement;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Tower>()
       .add_startup_system(generate_ui)
       .add_system(tower_shooting)
       .add_system(tower_button_clicked);
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

#[derive(EnumIter, Inspectable, Component, Display, Clone, Copy, Debug, PartialEq)]
pub enum TowerType {
  Nature,
  Fire,
  Ice,
  Dark,
  Mage,
  Archmage
}

impl TowerType {
  pub fn get_tower(&self, assets: &GameAssets) -> (Tower, Handle<Image>) {
    match self {
      TowerType::Nature => (
        Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        assets.wizard_nature.clone()
      ),
      TowerType::Fire => (
        Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        assets.wizard_fire.clone()
      ),
      TowerType::Ice => (
        Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        assets.wizard_ice.clone()
      ),
      TowerType::Dark => (
        Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        assets.wizard_dark.clone()
      ),
      TowerType::Mage => (
        Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        assets.wizard_mage.clone()
      ),
      TowerType::Archmage => (
        Tower::new(
          Vec3::new(20., 0., 0.),
          1,
          Timer::from_seconds(1., TimerMode::Repeating),
          10,
          100
        ),
        assets.wizard_archmage.clone()
      )
    }
  }
  
  pub fn get_bullet(&self, damage: i32, assets: &GameAssets) -> (Bullet, Movement, Handle<Image>) {
    match self {
      TowerType::Nature => (
        Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        assets.wizard_nature_bullet.clone()
      ),
      TowerType::Fire => (
        Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        assets.wizard_fire_bullet.clone()
      ),
      TowerType::Ice => (
        Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        assets.wizard_ice_bullet.clone()
      ),
      TowerType::Dark => (
        Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        assets.wizard_dark_bullet.clone()
      ),
      TowerType::Mage => (
        Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        assets.wizard_mage_bullet.clone()
      ),
      TowerType::Archmage => (
        Bullet {
          damage,
          lifetime: Timer::from_seconds(2., TimerMode::Once),
        },
        Movement {
          direction: Vec3::new(0.00000001,0.,0.),
          speed: 1500.
        },
        assets.wizard_archmage_bullet.clone()
      )
    }
  }
  
  pub fn path(&self) -> &str {
    match self {
      TowerType::Nature => "tower_buttons/wizard_nature_button.png",
      TowerType::Fire => "tower_buttons/wizard_fire_button.png",
      TowerType::Ice => "tower_buttons/wizard_ice_button.png",
      TowerType::Dark => "tower_buttons/wizard_dark_button.png",
      TowerType::Mage => "tower_buttons/wizard_mage_button.png",
      TowerType::Archmage => "tower_buttons/wizard_archmage_button.png",
      
    }
  }
}

// fn tower_spawn(
//   commands: &mut Commands,
//   assets: &GameAssets,
//   position: Vec3,
//   tower_type: TowerType,
// ) -> Entity {
//   let (tower_scene, tower) = tower_type.get_tower(assets);
//   commands
//     .spawn(SpatialBundle::from_transform(Transform::from_translation(
//       position,
//     )))
//     .insert(Name::new(format!("{:?}_Tower", tower_type)))
//     .insert(tower_type)
//     .insert(tower)
//     .with_children(|commands| {
//       commands.spawn(SceneBundle {
//         scene: tower_scene,
//         transform: Transform::from_xyz(0.0, -0.8, 0.0),
//         ..Default::default()
//       });
//     })
//     .id()
// }

fn tower_shooting(
  mut commands: Commands,
  assets: Res<GameAssets>, // Bullet assets
  mut towers: Query<(Entity, &mut Tower, &mut Transform, &GlobalTransform)>,
  enemies: Query<&GlobalTransform, With<Enemy>>, // Gets all entities With the Enemy component
  time: Res<Time>,
) {
  for (tower_entity, mut tower, mut tower_transform, transform) in &mut towers {
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
        
        // Make bullet a child of tower
        commands.entity(tower_entity).with_children(|commands| {
          commands.spawn(SpriteBundle {
            texture: assets.bullet.clone(),
            transform: bullet_transform,
            sprite: Sprite {
              custom_size: Some(Vec2::new(40., 22.)),
              ..default()
            },
            ..default()
          })
            .insert(Bullet {
              damage: tower.damage,
              lifetime: Timer::from_seconds(2., TimerMode::Once)
            })
            .insert(Movement {
              direction: Vec3::new(0.00000001,0.,0.),
              speed: 1500.,
            })
            .insert(Name::new("Bullet"));
        });
      }
    }
  }
}

// Marker component to despawn buttons in UI
#[derive(Component)]
pub struct TowerUIRoot;

fn tower_button_clicked(interaction: Query<(&Interaction, &TowerType), Changed<Interaction>>) {
  for (interaction, tower_type) in &interaction {
    match interaction {
      Interaction::Clicked => {
        //image = assets.wizard_fire_button_press.clone().into();
        info!("Spawning: {tower_type} wizard");
      }
      Interaction::Hovered => {
        //image = assets.wizard_fire_button_hover.clone().into();
      }
      Interaction::None => {
        //image = assets.wizard_fire_button.clone().into();
      }
    }
  }
}

// Creating a UI menu on the whole screen with buttons
fn generate_ui(mut commands: Commands, assets_server: Res<AssetServer>) {
  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    })
    .insert(TowerUIRoot) // Marker component
    .with_children(|commands| { // Make the buttons children of the menu
      for i in TowerType::iter() {
        commands
          .spawn(ButtonBundle {
            style: Style {
              size: Size::new(Val::Percent(15.0 * 9.0 / 16.0), Val::Percent(10.0)),
              align_self: AlignSelf::FlexEnd, // Bottom of screen
              margin: UiRect::all(Val::Percent(2.0)),
              ..default()
            },
            image: assets_server.load(i.path()).clone().into(),
            ..default()
          })
          .insert(i);
      }
    });
}

// // Creating a UI menu on the whole screen with buttons
// fn generate_ui(mut commands: Commands, assets_server: Res<AssetServer>) {
//   commands
//     .spawn(NodeBundle {
//       style: Style {
//         size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//         justify_content: JustifyContent::Center,
//         ..default()
//       },
//       ..default()
//     })
//     .insert(TowerUIRoot) // Marker component
//     .with_children(|commands| { // Make the buttons children of the menu
//       for i in TowerType::iter() {
//         commands
//           .spawn(ButtonBundle {
//             style: Style {
//               size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),
//               align_self: AlignSelf::FlexEnd, // Bottom of screen
//               margin: UiRect::all(Val::Percent(2.0)),
//               ..default()
//             },
//             image: assets_server.load(i.path()).clone().into(),
//             ..default()
//           })
//           .insert(i);
//       }
//     });
// }