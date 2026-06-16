use bevy::math::Vec3Swizzles;
use bevy::ecs::system::ParamSet;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
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
      .add_system(spawn_render_warmup.in_schedule(OnEnter(GameState::Gameplay)))
      .add_system(despawn_render_warmup.in_schedule(OnExit(GameState::Gameplay)))
      .add_systems(
        (
          tower_button_interaction.run_if(game_not_paused),
          place_tower.run_if(game_not_paused),
          close_upgrade_ui_while_placing.run_if(game_not_paused),
          tick_placement_error,
          snap_button_clicked,
          update_snap_button,
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

/// Marker on the translucent range circle that is a CHILD of the preview sprite.
/// Kept on a separate entity (not the sprite) so the sprite pipeline and the
/// mesh2d pipeline never share one entity - that hybrid caused a one-frame flash
/// where the range mesh sampled the tower texture across the whole map.
#[derive(Component)]
pub struct SpriteFollowerRange;

/// Marker on the invisible warm-up entities. See `spawn_render_warmup`.
#[derive(Component)]
pub struct RenderWarmup;

// The first frame a given tower texture (or the range mesh2d material) enters the
// render batch, Bevy 0.10 can mis-bind for a single frame - tiles briefly show the
// tower sprite on the very first selection after launch. Spawning fully transparent
// copies of every tower texture and one range mesh when gameplay begins forces those
// GPU resources to be uploaded and batched ahead of time, so the first real preview
// is already warm. The entities are alpha 0 and scaled to ~nothing, so they are never
// visible; they live for the gameplay session and are removed on exit.
fn spawn_render_warmup(
  mut commands: Commands,
  assets: Res<GameAssets>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for tower_type in TowerType::iter() {
    commands.spawn((
      SpriteBundle {
        texture: assets.get_tower_asset(tower_type),
        sprite: Sprite { color: Color::rgba(1.0, 1.0, 1.0, 0.0), ..default() },
        // Centered (so it is inside the camera frustum and actually gets rendered),
        // but alpha 0 and effectively zero-size, so it is imperceptible.
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -20.0))
          .with_scale(Vec3::splat(0.001)),
        ..default()
      },
      RenderWarmup,
      Name::new("RenderWarmup Sprite"),
    ));
  }

  // Warm the mesh2d + ColorMaterial pipeline used by the preview range circle.
  commands.spawn((
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Circle::new(1.0).into()).into(),
      material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.0))),
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, -20.0)),
      ..default()
    },
    RenderWarmup,
    Name::new("RenderWarmup Mesh"),
  ));
}

fn despawn_render_warmup(mut commands: Commands, warmups: Query<Entity, With<RenderWarmup>>) {
  for entity in &warmups {
    commands.entity(entity).despawn_recursive();
  }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TowerButtonState {
  price: u32,
}

#[derive(Component)]
pub struct SnapButton;

#[derive(Component)]
pub struct SnapButtonText;

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

#[derive(Resource, Default)]
struct PlacementState {
  cursor_exited_ui: bool,
  snap_mode: bool,
}

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

// How far the auto-nudge will search outward from the cursor for a valid spot.
const NUDGE_MAX_RADIUS: f32 = 160.0;
const NUDGE_STEP: f32 = 5.0;

// Is `pos` a fully legal placement: allowed terrain AND clear of every tower's radius?
fn placement_is_valid(
  pos: Vec2,
  new_radius: f32,
  tower_data: &[(Vec2, f32)],
  allowed_terrain: &[TerrainType],
  tiles: &Query<&MapTile>,
) -> bool {
  let terrain_ok = tile_at_world_pos(pos, tiles)
    .and_then(|t| t.terrain_type())
    .map(|t| allowed_terrain.contains(&t))
    .unwrap_or(false);
  terrain_ok && !tower_data.iter().any(|&(tp, r)| tp.distance(pos) <= new_radius + r)
}

// BTD6-style auto-nudge: if the cursor itself is a legal spot, return it unchanged
// (so the preview tracks the cursor smoothly). Otherwise search outward in growing
// rings and return the closest legal position found. If nothing legal is within
// NUDGE_MAX_RADIUS, return the cursor unchanged (placement stays invalid / red).
fn find_nearest_valid(
  cursor: Vec2,
  new_radius: f32,
  tower_data: &[(Vec2, f32)],
  allowed_terrain: &[TerrainType],
  tiles: &Query<&MapTile>,
) -> Vec2 {
  if placement_is_valid(cursor, new_radius, tower_data, allowed_terrain, tiles) {
    return cursor;
  }

  let mut radius = NUDGE_STEP;
  while radius <= NUDGE_MAX_RADIUS {
    // More angular samples on bigger rings keeps the search density roughly even.
    let samples = ((std::f32::consts::TAU * radius / NUDGE_STEP).ceil() as usize).max(8);
    let mut best: Option<(f32, Vec2)> = None;
    for i in 0..samples {
      let angle = i as f32 / samples as f32 * std::f32::consts::TAU;
      let p = cursor + Vec2::new(angle.cos(), angle.sin()) * radius;
      if placement_is_valid(p, new_radius, tower_data, allowed_terrain, tiles) {
        let d = cursor.distance_squared(p);
        if best.map_or(true, |(bd, _)| d < bd) {
          best = Some((d, p));
        }
      }
    }
    // First ring containing any valid point holds the nearest valid spot. The ring
    // point itself sits on a discrete grid, which makes the preview hop as the cursor
    // moves. Refine by walking from that point back toward the cursor to the exact
    // obstacle boundary - that boundary point slides smoothly and removes the jitter.
    if let Some((_, p)) = best {
      return refine_toward_cursor(cursor, p, new_radius, tower_data, allowed_terrain, tiles);
    }
    radius += NUDGE_STEP;
  }

  cursor
}

// `valid` is a legal spot, `cursor` is not. Binary-search the segment between them for
// the legal point closest to the cursor (i.e. right on the obstacle boundary). This
// makes the nudged position vary continuously with the cursor instead of snapping to
// the discrete ring-search grid.
fn refine_toward_cursor(
  cursor: Vec2,
  valid: Vec2,
  new_radius: f32,
  tower_data: &[(Vec2, f32)],
  allowed_terrain: &[TerrainType],
  tiles: &Query<&MapTile>,
) -> Vec2 {
  let mut lo = 0.0; // cursor end (invalid)
  let mut hi = 1.0; // valid end
  for _ in 0..16 {
    let mid = 0.5 * (lo + hi);
    if placement_is_valid(cursor.lerp(valid, mid), new_radius, tower_data, allowed_terrain, tiles) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  cursor.lerp(valid, hi)
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
    (Entity, &mut Transform, &TowerType, &mut Visibility),
    (With<SpriteFollower>, Without<GameplayUIRoot>),
  >,
  assets: Res<GameAssets>,
  mouse: Res<Input<MouseButton>>,
  keys: Res<Input<KeyCode>>,
  windows: Query<&Window>,
  camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut player: Query<&mut Player>,
  mut tower_tile_queries: ParamSet<(
    Query<(&Transform, &TowerType), (With<Tower>, Without<SpriteFollower>)>,
    Query<&MapTile>,
    Query<&mut Handle<ColorMaterial>, With<SpriteFollowerRange>>,
  )>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  game_data: Res<GameData>,
  tower_stats: Res<Assets<TowerTypeStats>>,
  node_query: Query<(&Node, &GlobalTransform, &Visibility), With<GameplayUIRoot>>,
  mut state: ResMut<PlacementState>,
  mut placement_error: ResMut<PlacementError>,
) {
  let Some(tower_stats) = tower_stats.get(&game_data.tower_type_stats)
    else { return; };

  let window = windows.get_single().unwrap();
  let (camera, camera_transform) = camera_query.single();
  let mut player = player.single_mut();

  // G toggles snap mode - works even before entering the entity loop.
  if keys.just_pressed(KeyCode::G) {
    state.snap_mode = !state.snap_mode;
    placement_error.message = if state.snap_mode {
      "Auto-nudge ON  - snaps to nearest valid spot  (G to toggle)".to_string()
    } else {
      "Auto-nudge OFF".to_string()
    };
    placement_error.timer = 2.0;
  }

  for (entity, mut transform, tower_type, mut vis) in query.iter_mut() {
    *vis = Visibility::Inherited; // un-hide after spawn frame; prevents 1-frame flash at world origin
    let allowed_terrain = &tower_stats.tower[tower_type].allowed_terrain.terrain;

    if let Some(position) = window.cursor_position() {
      if !cursor_above_ui(window, &node_query) {
        state.cursor_exited_ui = true;
      }

      let raw_pos = window_to_world_pos(window, position, camera, camera_transform).truncate();

      // Collect tower data once - releases p0 borrow before we need p1.
      let tower_data: Vec<(Vec2, f32)> = tower_tile_queries.p0()
        .iter()
        .map(|(t, tt)| (t.translation.truncate(), tt.placement_radius()))
        .collect();

      let new_radius = tower_type.placement_radius();
      let final_pos = if state.snap_mode {
        let tiles = tower_tile_queries.p1();
        find_nearest_valid(raw_pos, new_radius, &tower_data, allowed_terrain, &tiles)
      } else {
        raw_pos
      };
      transform.translation = final_pos.extend(0.5);

      // Overlap rule is IDENTICAL in snap and normal mode: each tower's radius counts.
      let tile_occupied = tower_data.iter().any(|&(tp, r)| {
        tp.distance(final_pos) <= new_radius + r
      });

      let terrain_ok = {
        let tiles = tower_tile_queries.p1();
        tile_at_world_pos(final_pos, &tiles)
          .and_then(|t| t.terrain_type())
          .map(|t| allowed_terrain.contains(&t))
          .unwrap_or(false)
      };

      let tint = match (tile_occupied || !terrain_ok, state.snap_mode) {
        (true, _)      => Color::rgba_u8(202, 0, 0, 150),   // invalid - red
        (false, true)  => Color::rgba_u8(0, 120, 220, 110), // snap + valid - blue
        (false, false) => Color::rgba_u8(0, 0, 0, 85),      // normal valid - dark
      };
      let new_material = materials.add(ColorMaterial::from(tint));
      for mut handle in tower_tile_queries.p2().iter_mut() {
        *handle = new_material.clone();
      }
    }

    if mouse.just_pressed(MouseButton::Left) && !cursor_above_ui(window, &node_query) {
      if let Some(screen_pos) = window.cursor_position() {
        state.cursor_exited_ui = false;
        let raw_click = window_to_world_pos(window, screen_pos, camera, camera_transform).truncate();

        let tower_data: Vec<(Vec2, f32)> = tower_tile_queries.p0()
          .iter()
          .map(|(t, tt)| (t.translation.truncate(), tt.placement_radius()))
          .collect();

        let new_radius = tower_type.placement_radius();
        let final_click = if state.snap_mode {
          let tiles = tower_tile_queries.p1();
          find_nearest_valid(raw_click, new_radius, &tower_data, allowed_terrain, &tiles)
        } else {
          raw_click
        };

        // Same overlap rule as the preview above - snap and normal mode are identical.
        let tile_occupied = tower_data.iter().any(|&(tp, r)| {
          tp.distance(final_click) <= new_radius + r
        });

        let error = {
          let tiles = tower_tile_queries.p1();
          placement_error_reason(final_click, allowed_terrain, tile_occupied, &tiles)
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
            final_click.extend(0.5),
            &mut meshes,
            &mut materials,
            tower_stats,
          );
        }
      }
    } else if mouse.just_pressed(MouseButton::Right)
      || window.cursor_position().is_none()
      || (state.cursor_exited_ui && cursor_above_ui(window, &node_query))
    {
      state.cursor_exited_ui = false;
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
      state.cursor_exited_ui = false;
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
    let world_pos = window_to_world_pos(window, position, camera, camera_transform);
    let initial_transform = Transform::from_translation(world_pos)
      .with_scale(Vec3::splat(tower_type.sprite_scale()));
    // Range circle is spawned as a CHILD (matches spawn_tower) so the mesh2d entity
    // is separate from the sprite entity. A combined sprite+mesh2d entity flashed the
    // tower texture across the whole map on the first frame.
    let range = spawn_tower_range(meshes, materials, tower_stats.tower[tower_type].tower.range);
    commands
      .spawn(SpriteBundle {
        texture: assets.get_tower_asset(*tower_type),
        transform: initial_transform,
        // Hidden on spawn so GlobalTransform (identity until propagation runs) never
        // reaches the renderer. place_tower makes it visible on its first tick; the
        // child range inherits this visibility.
        visibility: Visibility::Hidden,
        ..default()
      })
      .insert(SpriteFollower)
      .insert(*tower_type)
      .insert(Name::new("SpriteFollower"))
      .with_children(|parent| {
        parent
          .spawn(range)
          .insert(SpriteFollowerRange)
          .insert(Name::new("SpriteFollower Range"));
      });
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

fn snap_button_clicked(
  interactions: Query<&Interaction, (With<SnapButton>, Changed<Interaction>)>,
  mut state: ResMut<PlacementState>,
  mut placement_error: ResMut<PlacementError>,
) {
  for interaction in &interactions {
    if matches!(interaction, Interaction::Clicked) {
      state.snap_mode = !state.snap_mode;
      placement_error.message = if state.snap_mode {
        "Auto-nudge ON  - snaps to nearest valid spot  (G to toggle)".to_string()
      } else {
        "Auto-nudge OFF".to_string()
      };
      placement_error.timer = 2.0;
    }
  }
}

fn update_snap_button(
  state: Res<PlacementState>,
  mut buttons: Query<&mut BackgroundColor, With<SnapButton>>,
  mut texts: Query<&mut Text, With<SnapButtonText>>,
) {
  let (bg, label, text_color) = if state.snap_mode {
    (Color::rgba(0.0, 0.47, 0.87, 0.92), "Nudge: ON  [G]", Color::WHITE)
  } else {
    (Color::rgba(0.08, 0.08, 0.08, 0.80), "Nudge: OFF [G]", Color::rgb(0.55, 0.55, 0.55))
  };
  for mut bg_color in &mut buttons {
    *bg_color = BackgroundColor(bg);
  }
  for mut text in &mut texts {
    text.sections[0].value = label.to_string();
    text.sections[0].style.color = text_color;
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

  commands.insert_resource(PlacementState::default());
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

  // Snap toggle button = bottom-right corner, always visible
  commands
    .spawn(ButtonBundle {
      style: Style {
        position_type: PositionType::Absolute,
        position: UiRect { right: Val::Px(12.), bottom: Val::Px(12.), ..default() },
        padding: UiRect { left: Val::Px(10.), right: Val::Px(10.), top: Val::Px(6.), bottom: Val::Px(6.) },
        ..default()
      },
      background_color: BackgroundColor(Color::rgba(0.08, 0.08, 0.08, 0.80)),
      ..default()
    })
    .insert(TowerUIRoot)
    .insert(SnapButton)
    .insert(Name::new("SnapButton"))
    .with_children(|c| {
      c.spawn(TextBundle {
        text: Text::from_section(
          "Nudge: OFF [G]",
          TextStyle { font: assets.font.clone(), font_size: 15.0, color: Color::rgb(0.55, 0.55, 0.55) },
        ),
        ..default()
      })
      .insert(SnapButtonText);
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
