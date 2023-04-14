use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::assets::*;
use crate::GameState;
use crate::tower::*;

pub struct TowerUIPlugin;

impl Plugin for TowerUIPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(update_tower_ui.in_set(OnUpdate(GameState::Gameplay)));
  }
}

#[derive(Component)]
pub struct TowerUI;

#[derive(Component)]
pub struct TowerStatsUI;

#[derive(Component)]
pub struct TowerLifetimeStatsUI;

#[derive(Component)]
pub struct SellButton;

#[derive(Component)]
pub struct SellButtonText;

#[derive(Component)]
pub struct PreviousTargetingPriorityButton;

#[derive(Component)]
pub struct TargetingPriorityUI;

#[derive(Component)]
pub struct NextTargetingPriorityButton;

#[derive(Component)]
pub struct TowerUpgradeButton {
  pub path_index: usize
}

#[derive(Component)]
pub struct TowerUpgradeIndex;

fn update_tower_ui(
  mut child_q: Query<&Parent, With<TowerUpgradeUI>>,
  mut parent_q: Query<&mut Tower>,
  mut stats_ui: Query<&mut Text, With<TowerStatsUI>>,
  mut lifetime_stats_ui: Query<&mut Text, (With<TowerLifetimeStatsUI>, Without<TowerStatsUI>)>,
  mut targeting_priority_ui: Query<&mut Text,
    (With<TargetingPriorityUI>, Without<TowerStatsUI>, Without<TowerLifetimeStatsUI>)>,
  mut sell_button_ui: Query<&mut Text,
    (With<SellButtonText>, Without<TowerStatsUI>,
     Without<TowerLifetimeStatsUI>, Without<TargetingPriorityUI>)>
) {
  for parent in child_q.iter_mut() {
    let tower = parent_q.get_mut(parent.get()).unwrap();
    
    // Update tower stats
    for mut stats in stats_ui.iter_mut() {
      *stats = Text::from_section(
        format!(" Damage: {}\n Attack Speed: {}\n Range: {}\n Pierce: \n Projectile Speed: ",
                tower.damage, tower.attack_speed, tower.range),
        stats.sections[0].style.clone(),
      );
    }
    
    // Update tower lifetime stats
    for mut lifetime_stats in lifetime_stats_ui.iter_mut() {
      *lifetime_stats = Text::from_section(
        format!(" Total Damage: {}  Total Spent: ${}",
                tower.total_damage, tower.total_spent),
        lifetime_stats.sections[0].style.clone(),
      );
    }
    
    // Update targeting priority
    for mut targeting_priority in targeting_priority_ui.iter_mut() {
      *targeting_priority = Text::from_section(
        format!("{:?}", tower.target),
        targeting_priority.sections[0].style.clone(),
      );
    }
    
    
    // Update selling price
    for mut sell_text in sell_button_ui.iter_mut() {
      *sell_text = Text::from_section(
        format!("Sell: ${:?}", tower.sell_price),
        sell_text.sections[0].style.clone(),
      );
    }
  }
}

pub fn spawn_tower_range(
  meshes: &mut Assets<Mesh>,
  materials: &mut Assets<ColorMaterial>,
  radius: u32
) -> MaterialMesh2dBundle<ColorMaterial> {
  MaterialMesh2dBundle {
    mesh: meshes.add(shape::Circle::new(radius as f32).into()).into(),
    material: materials.add(ColorMaterial::from(
      Color::rgba_u8(0, 0, 0, 85))),
    transform: Transform::from_translation(Vec3::new(0., 0., -0.5)),
    ..default()
  }
}

pub fn spawn_tower_ui(
  commands: &mut Commands,
  assets: &GameAssets,
  tower: &Tower,
  tower_type: TowerType
) {
  commands
    .spawn(NodeBundle {
      background_color: BackgroundColor(Color::ORANGE),
      style: Style {
        size: Size::new(Val::Percent(20.), Val::Percent(40.)),
        position_type: PositionType::Absolute,
        position: UiRect {
          left: Val::Percent(80.),
          ..default()
        },
        justify_content: JustifyContent::FlexStart,
        align_self: AlignSelf::Center,
        flex_wrap: FlexWrap::Wrap,
        align_content: AlignContent::FlexStart,
        ..default()
      },
      ..default()
    })
    .with_children(|commands| {
      // Tower Icon
      commands.spawn(ImageBundle {
        style: Style {
          size: Size::new(Val::Px(100.), Val::Px(100.)),
          margin: UiRect {
            top: Val::Percent(5.),
            left: Val::Percent(5.),
            right: Val::Percent(5.),
            ..default()
          },
          ..default()
        },
        image: assets.get_tower_icon(tower_type).clone().into(),
        ..default()
      }).insert(TowerUI)
        .insert(Name::new("TowerIcon"));
      
      // Tower Stats
      commands.spawn(NodeBundle {
        background_color: BackgroundColor(Color::CRIMSON),
        style: Style {
          size: Size::new(Val::Percent(50.), Val::Percent(35.)),
          align_items: AlignItems::Center,
          margin: UiRect::top(Val::Percent(2.5)),
          ..default()
        },
        ..default()
      }).with_children(|commands| {
        commands.spawn(TextBundle {
          text: Text::from_section(
            "",
            TextStyle {
              font: assets.font.clone(),
              font_size: 17.0,
              color: Color::WHITE,
            },
          ),
          ..default()
        })
          .insert(TowerUI)
          .insert(TowerStatsUI)
          .insert(Name::new("TowerStatsText"));
      }).insert(TowerUI)
        .insert(Name::new("TowerStats"));
      
      // Total damage and total spent
      commands.spawn(NodeBundle {
        background_color: BackgroundColor(Color::RED),
        style: Style {
          size: Size::new(Val::Percent(100.), Val::Percent(4.5)),
          position_type: PositionType::Absolute,
          ..default()
        },
        ..default()
      }).with_children(|commands| {
        commands.spawn(TextBundle {
          text: Text::from_section(
            "",
            TextStyle {
              font: assets.font.clone(),
              font_size: 12.5,
              color: Color::WHITE,
            },
          ),
          ..default()
        }).insert(TowerUI)
          .insert(TowerLifetimeStatsUI)
          .insert(Name::new("TowerLifetimeStatsText"));
      }).insert(TowerUI)
        .insert(Name::new("TowerLifetimeStats"));
      
      // Targeting priority and sell button node
      commands.spawn(NodeBundle {
        style: Style {
          size: Size::new(Val::Percent(100.), Val::Percent(10.)),
          margin: UiRect {
            top: Val::Percent(1.),
            left: Val::Percent(2.),
            ..default()
          },
          align_items: AlignItems::Center,
          ..default()
        },
        ..default()
      }).with_children(|commands| {
        // Previous targeting priority button
        commands
          .spawn(ButtonBundle {
            style: Style {
              size: Size::new(Val::Px(26.), Val::Px(21.)),
              //align_self: AlignSelf::Center,
              margin: UiRect {
                left: Val::Percent(2.5),
                right: Val::Percent(2.),
                ..default()
              },
              ..default()
            },
            image: assets.prev_target_button.clone().into(),
            ..default()
          }).insert(TowerUI)
          .insert(PreviousTargetingPriorityButton)
          .insert(Name::new("PreviousButton"));
        
        // Targeting priority text
        commands.spawn(NodeBundle {
          style: Style {
            size: Size::new(Val::Percent(27.5), Val::Percent(15.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
          },
          ..default()
        }).with_children(|commands| {
          commands.spawn(TextBundle {
            text: Text::from_section(
              "",
              TextStyle {
                font: assets.font.clone(),
                font_size: 21.,
                color: Color::WHITE,
              },
            ),
            ..default()
          })
            .insert(TowerUI)
            .insert(TargetingPriorityUI)
            .insert(Name::new("TargetingPriorityText"));
        }).insert(TowerUI)
          .insert(Name::new("TargetingPriorityUI"));
        
        // Next targeting priority button
        commands.spawn(ButtonBundle {
          style: Style {
            size: Size::new(Val::Px(26.), Val::Px(21.)),
            //align_self: AlignSelf::Center,
            margin: UiRect {
              left: Val::Percent(2.),
              ..default()
            },
            ..default()
          },
          image: assets.next_target_button.clone().into(),
          ..default()
        }).insert(TowerUI)
          .insert(NextTargetingPriorityButton)
          .insert(Name::new("NextButton"));
        
        // Sell button
        commands.spawn(ButtonBundle {
          style: Style {
            size: Size::new(Val::Px(80.), Val::Px(30.)),
            margin: UiRect {
              left: Val::Percent(6.5),
              ..default()
            },
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
          },
          image: assets.sell_button.clone().into(),
          ..default()
        })
          .with_children(|commands| {
            commands.spawn(TextBundle {
              text: Text::from_section(
                "",
                TextStyle {
                  font: assets.font.clone(),
                  font_size: 16.5,
                  color: Color::WHITE,
                },
              ),
              ..default()
            }).insert(TowerUI)
              .insert(SellButtonText);
          }).insert(TowerUI)
          .insert(SellButton)
          .insert(Name::new("SellButton"));
      }).insert(TowerUI)
        .insert(Name::new("TargetingPriorityAndSellButton"));
      
      // Upgrades
      commands.spawn(NodeBundle {
        style: Style {
          size: Size::new(Val::Percent(100.), Val::Percent(42.5)),
          flex_wrap: FlexWrap::Wrap,
          margin: UiRect {
            left: Val::Percent(5.),
            right: Val::Percent(5.),
            top: Val::Percent(3.),
            ..default()
          },
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          ..default()
        },
        ..default()
      }).with_children(|commands| {
        // Spawn upgrade paths
        for i in 0..3 {
          commands.spawn(NodeBundle {
            background_color: BackgroundColor(Color::ORANGE_RED),
            style: Style {
              size: Size::new(Val::Percent(100.), Val::Percent(30.)),
              ..default()
            },
            ..default()
          }).with_children(|commands| {
            commands.spawn(ImageBundle {
              style: Style {
                size: Size::new(Val::Percent(45.), Val::Px(20.)),
                position_type: PositionType::Absolute,
                position: UiRect::top(Val::Percent(-20.)),
                ..default()
              },
              image: assets.upgrades[tower.upgrades.upgrades[i]].clone().into(),
              ..default()
            }).insert(TowerUpgradeIndex);
            
            commands.spawn(ButtonBundle {
              style: Style {
                size: Size::new(Val::Px(80.), Val::Px(30.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                margin: UiRect::left(Val::Percent(62.5)),
                ..default()
              },
              image: assets.upgrade_button.clone().into(),
              ..default()
            }).with_children(|commands| {
              commands.spawn(TextBundle {
                text: Text::from_section(
                  "Upgrade",
                  TextStyle {
                    font: assets.font.clone(),
                    font_size: 16.5,
                    color: Color::WHITE,
                  },
                ),
                ..default()
              });
            }).insert(TowerUI)
              .insert(TowerUpgradeButton {
                path_index: i
            });
          }).insert(TowerUI)
            .insert(Name::new(format!("TowerUpgradePath {i}")));
        }
      })
        .insert(TowerUI)
        .insert(Name::new("TowerUpgradeUI"));
    }).insert(TowerUI)
    .insert(TowerUpgradeUI)
    .insert(Name::new("TowerUI"));
}