use bevy::math::Vec3Swizzles;
use bevy::ecs::system::ParamSet;
use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::assets::*;
use crate::tower::*;
use crate::FarmBehavior;
use crate::{GameData, GameState, GameplayUIRoot, MainCamera, MapTile, Player, Tile, TerrainType, game_not_paused};

pub struct TowerButtonPlugin;

impl Plugin for TowerButtonPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(generate_ui.in_schedule(OnEnter(GameState::Gameplay)))
      .add_systems(
        (
          tower_button_interaction.run_if(game_not_paused),
          place_tower.run_if(game_not_paused),
          close_upgrade_ui_while_placing.run_if(game_not_paused),
          tick_placement_error,
          lock_tower_buttons.after(generate_ui).run_if(game_not_paused),
          update_tooltip_position,
        )
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_system(cleanup_tower_ui.in_schedule(OnExit(GameState::Gameplay)));
  }
}

// Marker component to despawn buttons in UI
#[derive(Component)]
pub struct TowerUIRoot;

#[derive(Component)]
pub struct TooltipPanel;

#[derive(Component)]
pub struct TooltipHeader;

#[derive(Component)]
pub struct TooltipLabels;

#[derive(Component)]
pub struct TooltipBases;

#[derive(Component)]
pub struct TooltipMaxes;

#[derive(Component)]
pub struct SpriteFollower;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TowerButtonState {
  price: u32,
}

#[derive(Component)]
pub struct PlacementErrorPanel;

#[derive(Component)]
pub struct PlacementErrorText;

#[derive(Resource, Default)]
pub struct PlacementError {
  pub message: String,
  pub timer: f32,
}

fn lock_tower_buttons(
  mut buttons: Query<(&mut TowerButtonState, &TowerType)>,
  mut button_images: Query<(&mut UiImage, &TowerType)>,
  player: Query<&Player>,
  assets: Res<GameAssets>,
) {
  let player = player.single();

  for (state, tower_type) in &mut buttons {
    for (mut image, button_tower_type) in button_images.iter_mut() {
      if player.money >= state.price as usize {
        if button_tower_type == tower_type {
          image.texture = assets.get_button_asset(*tower_type);
        }
      } else if button_tower_type == tower_type {
        image.texture = assets.get_button_locked_asset(*tower_type);
      }
    }
  }
}

// Convert cursor position from window/screen position to world position
pub fn window_to_world_pos(
  window: &Window,
  cursor_pos: Vec2,
  camera: &Camera,
  camera_transform: &GlobalTransform,
) -> Vec3 {
  // get the size of the window
  let window_size = Vec2::new(window.width(), window.height());

  // convert screen position [0...<resolution>] to ndc [-1..1] (gpu coordinates)
  // Normalized device coordinates
  let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;

  // matrix for undoing the projection and camera transform
  let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

  // use it to convert ndc to world-space coordinates
  let mut world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

  world_pos.z = 0.5;

  world_pos
}

#[derive(Resource)]
struct CursorExitedUI(bool);

pub fn cursor_above_ui<T: Component>(
  window: &Window,
  node_query: &Query<(&Node, &GlobalTransform, &Visibility), With<T>>,
) -> bool {
  if let Some(pointer_position) = window.cursor_position() {
    for (node, global_transform, &visibility) in node_query.iter() {
      if visibility == Visibility::Inherited {
        let node_position = global_transform.translation().xy();
        let half_size = 0.5 * Vec2::new(node.size().x, window.height() * 0.20);
        let min = node_position - half_size;
        let max = node_position + half_size;
        if (min.x..max.x).contains(&pointer_position.x)
          && (min.y..max.y).contains(&pointer_position.y)
        {
          return true;
        }
      }
    }
  }
  false
}

// Tiles are centered at (col*80, row*80), so a half-tile offset is needed.
fn tile_at_world_pos(world_pos: Vec2, map_tiles: &Query<&MapTile>) -> Option<Tile> {
  if world_pos.x < -40.0 || world_pos.y < -40.0 {
    return None;
  }
  let tile_x = ((world_pos.x + 40.0) / 80.0) as usize;
  let tile_y = ((world_pos.y + 40.0) / 80.0) as usize;
  map_tiles.iter()
    .find(|mt| mt.coordinate.x == tile_x && mt.coordinate.y == tile_y)
    .map(|mt| mt.tile.clone())
}

fn placement_error_reason(
  world_pos: Vec2,
  allowed_terrain: &[TerrainType],
  tower_overlap: bool,
  map_tiles: &Query<&MapTile>,
) -> Option<String> {
  if tower_overlap {
    return Some("A tower is already placed here".to_string());
  }
  match tile_at_world_pos(world_pos, map_tiles) {
    None => Some("Cannot place outside the map".to_string()),
    Some(tile) => match tile.terrain_type() {
      None => Some("Towers cannot be placed on the path".to_string()),
      Some(terrain) if allowed_terrain.contains(&terrain) => None,
      Some(_) => {
        let needed = allowed_terrain
          .iter()
          .map(|t| format!("{t:?}"))
          .collect::<Vec<_>>()
          .join(" or ");
        Some(format!("This tower can only be placed on: {needed}"))
      }
    },
  }
}

fn tick_placement_error(
  mut error: ResMut<PlacementError>,
  mut panels: Query<&mut Visibility, With<PlacementErrorPanel>>,
  mut texts: Query<&mut Text, With<PlacementErrorText>>,
  time: Res<Time>,
) {
  let showing = error.timer > 0.0;
  if showing {
    error.timer -= time.delta_seconds();
    for mut text in &mut texts {
      text.sections[0].value = error.message.clone();
    }
  }
  for mut vis in &mut panels {
    *vis = if showing { Visibility::Inherited } else { Visibility::Hidden };
  }
}

fn close_upgrade_ui_while_placing(
  mut commands: Commands,
  followers: Query<Entity, With<SpriteFollower>>,
  upgrade_uis: Query<Entity, With<TowerUpgradeUI>>,
) {
  if !followers.is_empty() {
    for entity in &upgrade_uis {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn place_tower(
  mut commands: Commands,
  mut query: Query<
    (
      Entity,
      &mut Transform,
      &TowerType,
      &mut Handle<ColorMaterial>,
    ),
    With<SpriteFollower>,
  >,
  assets: Res<GameAssets>,
  mouse: Res<Input<MouseButton>>,
  keys: Res<Input<KeyCode>>,
  windows: Query<&Window>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut player: Query<&mut Player>,
  mut tower_tile_queries: ParamSet<(
    Query<&Transform, (With<Tower>, Without<SpriteFollower>)>,
    Query<&MapTile>,
  )>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  game_data: Res<GameData>,
  tower_stats: Res<Assets<TowerTypeStats>>,
  node_query: Query<(&Node, &GlobalTransform, &Visibility), With<GameplayUIRoot>>,
  mut cursor_exited_ui: ResMut<CursorExitedUI>,
  mut placement_error: ResMut<PlacementError>,
) {
  let Some(tower_stats) = tower_stats.get(&game_data.tower_type_stats)
    else { return; };

  let window = windows.get_single().unwrap();
  let (camera, camera_transform) = camera_query.single();
  let mut player = player.single_mut();

  for (entity, mut transform, tower_type, mut color) in query.iter_mut() {
    let allowed_terrain = &tower_stats.tower[tower_type].allowed_terrain.terrain;

    if let Some(position) = window.cursor_position() {
      if !cursor_above_ui(window, &node_query) {
        cursor_exited_ui.0 = true;
      }

      transform.translation = window_to_world_pos(window, position, camera, camera_transform);
      let world_pos_2d = transform.translation.truncate();

      let tower_close = tower_tile_queries.p0()
        .iter()
        .any(|t| Vec3::distance(transform.translation, t.translation) <= 50.);

      let terrain_ok = {
        let tiles = tower_tile_queries.p1();
        tile_at_world_pos(world_pos_2d, &tiles)
          .and_then(|t| t.terrain_type())
          .map(|t| allowed_terrain.contains(&t))
          .unwrap_or(false)
      };

      if tower_close || !terrain_ok {
        *color = materials.add(ColorMaterial::from(Color::rgba_u8(202, 0, 0, 150)));
      } else {
        *color = materials.add(ColorMaterial::from(Color::rgba_u8(0, 0, 0, 85)));
      }
    }

    if mouse.just_pressed(MouseButton::Left) && !cursor_above_ui(window, &node_query) {
      if let Some(screen_pos) = window.cursor_position() {
        cursor_exited_ui.0 = false;
        let click_pos = window_to_world_pos(window, screen_pos, camera, camera_transform);

        let tower_overlap = tower_tile_queries.p0()
          .iter()
          .any(|t| Vec3::distance(click_pos, t.translation) <= 40.);

        let error = {
          let tiles = tower_tile_queries.p1();
          placement_error_reason(click_pos.truncate(), allowed_terrain, tower_overlap, &tiles)
        };

        if let Some(msg) = error {
          placement_error.message = msg;
          placement_error.timer = 2.5;
        } else {
          player.money -= tower_stats.tower[tower_type].tower.price as usize;
          commands.entity(entity).despawn_recursive();
          spawn_tower(
            &mut commands,
            *tower_type,
            &assets,
            click_pos,
            &mut meshes,
            &mut materials,
            tower_stats,
          );
        }
      }
    } else if mouse.just_pressed(MouseButton::Right)
      || window.cursor_position().is_none()
      || (cursor_exited_ui.0 && cursor_above_ui(window, &node_query))
    {
      cursor_exited_ui.0 = false;
      commands.entity(entity).despawn_recursive();
    } else if keys.just_pressed(KeyCode::Key1)
      || keys.just_pressed(KeyCode::Key2)
      || keys.just_pressed(KeyCode::Key3)
      || keys.just_pressed(KeyCode::Key4)
      || keys.just_pressed(KeyCode::Key5)
      || keys.just_pressed(KeyCode::Key6)
      || keys.just_pressed(KeyCode::Key7)
      || keys.just_pressed(KeyCode::Key8)
      || keys.just_pressed(KeyCode::Key9)
      || keys.just_pressed(KeyCode::Key0)
    {
      cursor_exited_ui.0 = false;
      commands.entity(entity).despawn_recursive();
      tower_spawn_from_keyboard_input(
        &mut commands,
        &keys,
        &player,
        window,
        camera,
        camera_transform,
        &mut meshes,
        &mut materials,
        &assets,
        tower_stats,
      );
    }
  }
}

fn spawn_sprite_follower(
  commands: &mut Commands,
  window: &Window,
  camera: &Camera,
  camera_transform: &GlobalTransform,
  meshes: &mut Assets<Mesh>,
  materials: &mut Assets<ColorMaterial>,
  tower_type: &TowerType,
  assets: &GameAssets,
  tower_stats: &TowerTypeStats,
) {
  // Spawn component that alerts the place_tower() system that a button has been pressed,
  // and it starts moving a sprite with the cursor until the tower is placed
  if let Some(position) = window.cursor_position() {
    let transform = window_to_world_pos(window, position, camera, camera_transform);
    commands
      .spawn(SpriteBundle {
        texture: assets.get_tower_asset(*tower_type),
        transform: Transform::from_translation(transform),
        ..default()
      })
      // .with_children(|commands| {
      //   commands.spawn(spawn_tower_range(meshes, materials,
      //                                    tower_stats.tower[&tower_type].tower.range))
      //     .insert(SpriteFollower)
      //     .insert(Name::new("Tower Range"));
      // })
      .insert(spawn_tower_range(
        meshes,
        materials,
        tower_stats.tower[tower_type].tower.range,
      ))
      .insert(SpriteFollower)
      .insert(*tower_type)
      .insert(Name::new("SpriteFollower"));
  }
}

fn tower_button_interaction(
  mut commands: Commands,
  assets: Res<GameAssets>,
  interaction: Query<
    (&Interaction, &TowerType, &TowerButtonState),
    (Changed<Interaction>, With<Button>),
  >,
  mut images: Query<(&mut UiImage, &TowerType)>,
  windows: Query<&Window>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  keys: Res<Input<KeyCode>>,
  query: Query<&SpriteFollower>,
  player: Query<&Player>,
  game_data: Res<GameData>,
  tower_stats: Res<Assets<TowerTypeStats>>,
  upgrades_res: Res<Assets<Upgrades>>,
  mut tooltip_panel: Query<&mut Visibility, With<TooltipPanel>>,
  mut tooltip_texts: ParamSet<(
    Query<&mut Text, With<TooltipHeader>>,
    Query<&mut Text, With<TooltipLabels>>,
    Query<&mut Text, With<TooltipBases>>,
    Query<&mut Text, With<TooltipMaxes>>,
  )>,
) {
  let Some(tower_stats) = tower_stats.get(&game_data.tower_type_stats)
    else { return; };

  let window = windows.get_single().unwrap();
  let (camera, camera_transform) = camera_query.single();
  let player = player.single();

  // Keyboard shortcuts
  if query.is_empty() {
    // Spawn one tower at a time
    tower_spawn_from_keyboard_input(
      &mut commands,
      &keys,
      player,
      window,
      camera,
      camera_transform,
      &mut meshes,
      &mut materials,
      &assets,
      tower_stats,
    );
  }

  for (interaction, tower_type, state) in &interaction {
    if player.money >= state.price as usize {
      match interaction {
        Interaction::Clicked => {
          if query.is_empty() {
            // Spawn one tower at a time
            // Change button UI
            for (mut image, button_tower_type) in images.iter_mut() {
              if button_tower_type == tower_type {
                image.texture = assets.get_button_pressed_asset(*tower_type);
              }
            }

            // Spawn tower sprite following mouse
            spawn_sprite_follower(
              &mut commands,
              window,
              camera,
              camera_transform,
              &mut meshes,
              &mut materials,
              tower_type,
              &assets,
              tower_stats,
            );
          }
        }
        Interaction::Hovered => {
          for (mut image, button_tower_type) in images.iter_mut() {
            if button_tower_type == tower_type {
              image.texture = assets.get_button_hovered_asset(*tower_type);
            }
          }
          let t = &tower_stats.tower[tower_type].tower;
          let (stats_labels, stats_bases, stats_maxes) = if let Some(all_upgrades) = upgrades_res.get(&game_data.tower_upgrades) {
            let paths = &all_upgrades.upgrades[tower_type];
            // Pick the path that contributes most damage (combat) or income (farm)
            let preview_path = paths.iter().max_by_key(|path| {
              path.iter().flat_map(|u| u.upgrade.iter())
                .map(|(s, v)| match s { TowerStat::Damage => v * 10, TowerStat::Income => *v, _ => 0 })
                .sum::<i32>()
            }).unwrap();

            let mut max_dmg = t.damage as i32;
            let mut max_spd = t.attack_speed;
            let mut max_rng = t.range as i32;
            let mut max_pierce = t.pierce as i32;
            let base_income = tower_type.get_farm_tower().map(|f| match f.behavior {
              FarmBehavior::Passive { income, .. }       => income,
              FarmBehavior::Kill { income_per_kill }     => income_per_kill,
              FarmBehavior::Wave { income_per_wave }     => income_per_wave,
              FarmBehavior::SelfKill { income_per_kill } => income_per_kill,
            }).unwrap_or(0);
            let mut max_income = base_income;

            for upgrade in preview_path {
              for (stat, val) in &upgrade.upgrade {
                match stat {
                  TowerStat::Damage        => max_dmg += *val,
                  TowerStat::AttackSpeed   => max_spd -= (*val as f32) * 0.01 * max_spd,
                  TowerStat::Range         => max_rng += *val,
                  TowerStat::Pierce        => max_pierce += *val,
                  TowerStat::Income        => max_income += *val as u32,
                  TowerStat::ProjectileSpeed => {}
                }
              }
            }

            if t.damage > 0 {
              let mut lbl = "DMG\nSPD\nRNG\nPierce".to_string();
              let mut bas = format!("{}\n{:.1}\n{}\n{}", t.damage, t.attack_speed, t.range, t.pierce);
              let mut maxs = format!("→ {}\n→ {:.1}\n→ {}\n→ {}", max_dmg, max_spd, max_rng, max_pierce);
              if base_income > 0 {
                lbl  += "\n$/kill";
                bas  += &format!("\n{}", base_income);
                maxs += &format!("\n→ {}", max_income);
              }
              (lbl, bas, maxs)
            } else {
              let (unit, base) = match tower_type.get_farm_tower().map(|f| f.behavior) {
                Some(FarmBehavior::Passive { income, .. })    => ("/15s", income),
                Some(FarmBehavior::Kill { income_per_kill })  => ("/kill", income_per_kill),
                Some(FarmBehavior::Wave { income_per_wave })  => ("/wave", income_per_wave),
                _                                             => ("", 0),
              };
              ("Income".to_string(), format!("${}{}", base, unit), format!("→ ${}{}", max_income, unit))
            }
          } else {
            if t.damage > 0 {
              ("DMG\nSPD\nRNG".to_string(),
               format!("{}\n{:.1}\n{}", t.damage, t.attack_speed, t.range),
               String::new())
            } else {
              (String::new(), String::new(), String::new())
            }
          };

          for mut text in tooltip_texts.p0().iter_mut() {
            text.sections[0].value = format!("{}\n{}\n", tower_stats.tower[tower_type].name, tower_type.description());
          }
          for mut text in tooltip_texts.p1().iter_mut() {
            text.sections[0].value = stats_labels.clone();
          }
          for mut text in tooltip_texts.p2().iter_mut() {
            text.sections[0].value = stats_bases.clone();
          }
          for mut text in tooltip_texts.p3().iter_mut() {
            text.sections[0].value = stats_maxes.clone();
          }
          for mut vis in tooltip_panel.iter_mut() {
            *vis = Visibility::Inherited;
          }
        }
        Interaction::None => {
          for (mut image, button_tower_type) in images.iter_mut() {
            if button_tower_type == tower_type {
              image.texture = assets.get_button_asset(*tower_type);
            }
          }
          for mut vis in tooltip_panel.iter_mut() {
            *vis = Visibility::Hidden;
          }
        }
      }
    }
  }
}

fn tower_spawn_from_keyboard_input(
  commands: &mut Commands,
  keys: &Input<KeyCode>,
  player: &Player,
  window: &Window,
  camera: &Camera,
  camera_transform: &GlobalTransform,
  meshes: &mut Assets<Mesh>,
  materials: &mut Assets<ColorMaterial>,
  assets: &GameAssets,
  tower_stats: &TowerTypeStats,
) {
  if keys.just_pressed(KeyCode::Key1)
    && player.money >= tower_stats.tower[&TowerType::Nature].tower.price as usize
  {
    spawn_sprite_follower(
      commands,
      window,
      camera,
      camera_transform,
      meshes,
      materials,
      &TowerType::Nature,
      assets,
      tower_stats,
    );
  } else if keys.just_pressed(KeyCode::Key2)
    && player.money >= tower_stats.tower[&TowerType::Fire].tower.price as usize
  {
    spawn_sprite_follower(
      commands,
      window,
      camera,
      camera_transform,
      meshes,
      materials,
      &TowerType::Fire,
      assets,
      tower_stats,
    );
  } else if keys.just_pressed(KeyCode::Key3)
    && player.money >= tower_stats.tower[&TowerType::Ice].tower.price as usize
  {
    spawn_sprite_follower(
      commands,
      window,
      camera,
      camera_transform,
      meshes,
      materials,
      &TowerType::Ice,
      assets,
      tower_stats,
    );
  } else if keys.just_pressed(KeyCode::Key4)
    && player.money >= tower_stats.tower[&TowerType::Dark].tower.price as usize
  {
    spawn_sprite_follower(
      commands,
      window,
      camera,
      camera_transform,
      meshes,
      materials,
      &TowerType::Dark,
      assets,
      tower_stats,
    );
  } else if keys.just_pressed(KeyCode::Key5)
    && player.money >= tower_stats.tower[&TowerType::Mage].tower.price as usize
  {
    spawn_sprite_follower(
      commands,
      window,
      camera,
      camera_transform,
      meshes,
      materials,
      &TowerType::Mage,
      assets,
      tower_stats,
    );
  } else if keys.just_pressed(KeyCode::Key6)
    && player.money >= tower_stats.tower[&TowerType::Archmage].tower.price as usize
  {
    spawn_sprite_follower(
      commands,
      window,
      camera,
      camera_transform,
      meshes,
      materials,
      &TowerType::Archmage,
      assets,
      tower_stats,
    );
  } else if keys.just_pressed(KeyCode::Key7)
    && player.money >= tower_stats.tower[&TowerType::FarmPassive].tower.price as usize
  {
    spawn_sprite_follower(
      commands,
      window,
      camera,
      camera_transform,
      meshes,
      materials,
      &TowerType::FarmPassive,
      assets,
      tower_stats,
    );
  } else if keys.just_pressed(KeyCode::Key8)
    && player.money >= tower_stats.tower[&TowerType::FarmKill].tower.price as usize
  {
    spawn_sprite_follower(
      commands,
      window,
      camera,
      camera_transform,
      meshes,
      materials,
      &TowerType::FarmKill,
      assets,
      tower_stats,
    );
  } else if keys.just_pressed(KeyCode::Key9)
    && player.money >= tower_stats.tower[&TowerType::FarmWave].tower.price as usize
  {
    spawn_sprite_follower(
      commands,
      window,
      camera,
      camera_transform,
      meshes,
      materials,
      &TowerType::FarmWave,
      assets,
      tower_stats,
    );
  } else if keys.just_pressed(KeyCode::Key0)
    && player.money >= tower_stats.tower[&TowerType::FarmSelfKill].tower.price as usize
  {
    spawn_sprite_follower(
      commands,
      window,
      camera,
      camera_transform,
      meshes,
      materials,
      &TowerType::FarmSelfKill,
      assets,
      tower_stats,
    );
  }
}

// Creating a UI menu on the whole screen with buttons
fn cleanup_tower_ui(
  mut commands: Commands,
  roots: Query<Entity, With<TowerUIRoot>>,
  followers: Query<Entity, With<SpriteFollower>>,
) {
  for entity in &roots {
    commands.entity(entity).despawn_recursive();
  }
  for entity in &followers {
    commands.entity(entity).despawn_recursive();
  }
}

fn generate_ui(
  mut commands: Commands,
  assets: Res<GameAssets>,
  game_data: Res<GameData>,
  tower_stats: Res<Assets<TowerTypeStats>>,
) {
  let Some(tower_stats) = tower_stats.get(&game_data.tower_type_stats)
    else { return; };

  commands.insert_resource(CursorExitedUI(false));
  commands.insert_resource(PlacementError::default());

  // Error message = full-width row at top that flex-centers the dark panel
  commands
    .spawn(NodeBundle {
      style: Style {
        position_type: PositionType::Absolute,
        size: Size::new(Val::Percent(100.), Val::Auto),
        justify_content: JustifyContent::Center,
        position: UiRect { top: Val::Px(15.), ..default() },
        ..default()
      },
      ..default()
    })
    .insert(TowerUIRoot)
    .insert(Name::new("PlacementErrorWrapper"))
    .with_children(|c| {
      c.spawn(NodeBundle {
        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.85)),
        style: Style {
          padding: UiRect::all(Val::Px(10.)),
          ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
      })
      .insert(PlacementErrorPanel)
      .insert(Name::new("PlacementErrorPanel"))
      .with_children(|c| {
        c.spawn(TextBundle {
          text: Text::from_section(
            "",
            TextStyle {
              font: assets.font.clone(),
              font_size: 18.0,
              color: Color::rgb(1.0, 0.3, 0.3),
            },
          ),
          ..default()
        })
        .insert(PlacementErrorText);
      });
    });

  commands
    .spawn(NodeBundle {
      background_color: BackgroundColor(Color::GOLD),
      style: Style {
        size: Size::new(Val::Percent(100.), Val::Percent(12.)),
        justify_content: JustifyContent::Center,
        align_self: AlignSelf::FlexEnd,
        ..default()
      },
      ..default()
    })
    .insert(TowerUIRoot) // Marker component
    .insert(Name::new("TowerButtons"))
    .with_children(|commands| {
      // Make the buttons children of the menu
      for i in TowerType::iter() {
        commands
          .spawn(ButtonBundle {
            style: Style {
              size: Size::new(Val::Px(85.), Val::Px(80.)),
              align_self: AlignSelf::Center,
              justify_content: JustifyContent::Center,
              margin: UiRect {
                left: Val::Percent(2.),
                right: Val::Percent(2.),
                ..default()
              },
              ..default()
            },
            image: assets.get_button_asset(i).into(),
            ..default()
          })
          .with_children(|commands| {
            commands.spawn(TextBundle {
              text: Text::from_section(
                format!("${}", tower_stats.tower[&i].tower.price),
                TextStyle {
                  font: assets.font.clone(),
                  font_size: 30.0,
                  color: Color::YELLOW_GREEN,
                },
              ),
              style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
              },
              ..default()
            });
          })
          .insert(TowerButtonState {
            price: tower_stats.tower[&i].tower.price,
          })
          .insert(i)
          .insert(Name::new("TowerButton"));
      }
    });

  // Tooltip panel = hidden until a button is hovered; position updated by update_tooltip_position
  commands
    .spawn(NodeBundle {
      background_color: BackgroundColor(Color::rgba(0.05, 0.05, 0.05, 0.88)),
      style: Style {
        position_type: PositionType::Absolute,
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(10.)),
        ..default()
      },
      visibility: Visibility::Hidden,
      ..default()
    })
    .insert(TowerUIRoot)
    .insert(TooltipPanel)
    .insert(Name::new("TooltipPanel"))
    .with_children(|commands| {
      let style = TextStyle { font: assets.font.clone(), font_size: 16.0, color: Color::WHITE };
      commands.spawn(TextBundle {
        text: Text::from_section("", style.clone()),
        ..default()
      }).insert(TooltipHeader);
      commands.spawn(NodeBundle {
        style: Style { flex_direction: FlexDirection::Row, ..default() },
        ..default()
      }).with_children(|c| {
        c.spawn(TextBundle {
          text: Text::from_section("", style.clone()).with_alignment(TextAlignment::Right),
          ..default()
        }).insert(TooltipLabels);
        c.spawn(TextBundle {
          text: Text::from_section("", style.clone()).with_alignment(TextAlignment::Right),
          style: Style { min_size: Size::new(Val::Px(36.), Val::Auto), margin: UiRect::horizontal(Val::Px(4.)), ..default() },
          ..default()
        }).insert(TooltipBases);
        c.spawn(TextBundle {
          text: Text::from_section("", style.clone()),
          ..default()
        }).insert(TooltipMaxes);
      });
    });
}

fn update_tooltip_position(
  windows: Query<&Window>,
  mut tooltip: Query<(&mut Style, &Visibility), With<TooltipPanel>>,
) {
  let Ok(window) = windows.get_single() else { return; };
  let Ok((mut style, visibility)) = tooltip.get_single_mut() else { return; };
  if *visibility == Visibility::Hidden { return; }

  if let Some(cursor) = window.cursor_position() {
    // cursor_position() uses bottom-left origin, Y up = same as Val::Px bottom/left
    let x = cursor.x.min(window.width() - 340.);
    let y = cursor.y + 30.; // 30px above cursor
    style.position = UiRect {
      left: Val::Px(x),
      bottom: Val::Px(y),
      ..default()
    };
  }
}
