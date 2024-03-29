use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::movement::*;
use crate::{Tile, TowerType};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(animate_enemy_sprite)
      // Load assets before the startup stage, so we can use them in the game
      .add_startup_system(load_assets.in_base_set(StartupSet::PreStartup));
  }
}

// Asset loading

#[derive(Resource)]
pub struct GameAssets {
  // Fonts
  pub font: Handle<Font>,
  // Main menu buttons - Start Game & Exit
  pub start_button: Handle<Image>,
  pub exit_button: Handle<Image>,
  // Map tiles
  pub grass_tile: Handle<Image>,
  pub water_tile: Handle<Image>,
  pub path_tile: Handle<Image>,
  // Gameplay images - Health & Money
  pub heart: Handle<Image>,
  pub coin: Handle<Image>,
  // Towers
  pub wizard_nature: Handle<Image>,
  pub wizard_fire: Handle<Image>,
  pub wizard_ice: Handle<Image>,
  pub wizard_dark: Handle<Image>,
  pub wizard_mage: Handle<Image>,
  pub wizard_archmage: Handle<Image>,
  // Tower buttons
  pub wizard_nature_button: Handle<Image>,
  pub wizard_fire_button: Handle<Image>,
  pub wizard_ice_button: Handle<Image>,
  pub wizard_dark_button: Handle<Image>,
  pub wizard_mage_button: Handle<Image>,
  pub wizard_archmage_button: Handle<Image>,
  // Tower buttons hovered
  pub wizard_nature_button_hover: Handle<Image>,
  pub wizard_fire_button_hover: Handle<Image>,
  pub wizard_ice_button_hover: Handle<Image>,
  pub wizard_dark_button_hover: Handle<Image>,
  pub wizard_mage_button_hover: Handle<Image>,
  pub wizard_archmage_button_hover: Handle<Image>,
  // Tower buttons pressed
  pub wizard_nature_button_press: Handle<Image>,
  pub wizard_fire_button_press: Handle<Image>,
  pub wizard_ice_button_press: Handle<Image>,
  pub wizard_dark_button_press: Handle<Image>,
  pub wizard_mage_button_press: Handle<Image>,
  pub wizard_archmage_button_press: Handle<Image>,
  // Tower buttons locked
  pub wizard_nature_button_lock: Handle<Image>,
  pub wizard_fire_button_lock: Handle<Image>,
  pub wizard_ice_button_lock: Handle<Image>,
  pub wizard_dark_button_lock: Handle<Image>,
  pub wizard_mage_button_lock: Handle<Image>,
  pub wizard_archmage_button_lock: Handle<Image>,
  // Selected tower UI - Tower icons
  pub wizard_nature_icon: Handle<Image>,
  pub wizard_fire_icon: Handle<Image>,
  pub wizard_ice_icon: Handle<Image>,
  pub wizard_dark_icon: Handle<Image>,
  pub wizard_mage_icon: Handle<Image>,
  pub wizard_archmage_icon: Handle<Image>,
  // Selected tower UI - Targeting Priority buttons
  pub prev_target_button: Handle<Image>,
  pub next_target_button: Handle<Image>,
  // Selected tower UI - Sell button
  pub sell_button: Handle<Image>,
  // Selected tower UI - Upgrades
  pub upgrade_button: Handle<Image>,
  pub upgrades: [Handle<Image>; 6],
  // Enemies
  pub enemy: Handle<TextureAtlas>,
  // Tower bullets
  pub wizard_nature_bullet: Handle<Image>,
  pub wizard_fire_bullet: Handle<Image>,
  pub wizard_ice_bullet: Handle<Image>,
  pub wizard_dark_bullet: Handle<Image>,
  pub wizard_mage_bullet: Handle<Image>,
  pub wizard_archmage_bullet: Handle<Image>,
}

impl GameAssets {
  pub fn get_tower_asset(&self, tower_type: TowerType) -> Handle<Image> {
    match tower_type {
      TowerType::Nature => self.wizard_nature.clone(),
      TowerType::Fire => self.wizard_fire.clone(),
      TowerType::Ice => self.wizard_ice.clone(),
      TowerType::Dark => self.wizard_dark.clone(),
      TowerType::Mage => self.wizard_mage.clone(),
      TowerType::Archmage => self.wizard_archmage.clone(),
    }
  }

  pub fn get_tower_icon(&self, tower_type: TowerType) -> Handle<Image> {
    match tower_type {
      TowerType::Nature => self.wizard_nature_icon.clone(),
      TowerType::Fire => self.wizard_fire_icon.clone(),
      TowerType::Ice => self.wizard_ice_icon.clone(),
      TowerType::Dark => self.wizard_dark_icon.clone(),
      TowerType::Mage => self.wizard_mage_icon.clone(),
      TowerType::Archmage => self.wizard_archmage_icon.clone(),
    }
  }

  pub fn get_button_asset(&self, tower_type: TowerType) -> Handle<Image> {
    match tower_type {
      TowerType::Nature => self.wizard_nature_button.clone(),
      TowerType::Fire => self.wizard_fire_button.clone(),
      TowerType::Ice => self.wizard_ice_button.clone(),
      TowerType::Dark => self.wizard_dark_button.clone(),
      TowerType::Mage => self.wizard_mage_button.clone(),
      TowerType::Archmage => self.wizard_archmage_button.clone(),
    }
  }

  pub fn get_button_hovered_asset(&self, tower_type: TowerType) -> Handle<Image> {
    match tower_type {
      TowerType::Nature => self.wizard_nature_button_hover.clone(),
      TowerType::Fire => self.wizard_fire_button_hover.clone(),
      TowerType::Ice => self.wizard_ice_button_hover.clone(),
      TowerType::Dark => self.wizard_dark_button_hover.clone(),
      TowerType::Mage => self.wizard_mage_button_hover.clone(),
      TowerType::Archmage => self.wizard_archmage_button_hover.clone(),
    }
  }

  pub fn get_button_pressed_asset(&self, tower_type: TowerType) -> Handle<Image> {
    match tower_type {
      TowerType::Nature => self.wizard_nature_button_press.clone(),
      TowerType::Fire => self.wizard_fire_button_press.clone(),
      TowerType::Ice => self.wizard_ice_button_press.clone(),
      TowerType::Dark => self.wizard_dark_button_press.clone(),
      TowerType::Mage => self.wizard_mage_button_press.clone(),
      TowerType::Archmage => self.wizard_archmage_button_press.clone(),
    }
  }

  pub fn get_button_locked_asset(&self, tower_type: TowerType) -> Handle<Image> {
    match tower_type {
      TowerType::Nature => self.wizard_nature_button_lock.clone(),
      TowerType::Fire => self.wizard_fire_button_lock.clone(),
      TowerType::Ice => self.wizard_ice_button_lock.clone(),
      TowerType::Dark => self.wizard_dark_button_lock.clone(),
      TowerType::Mage => self.wizard_mage_button_lock.clone(),
      TowerType::Archmage => self.wizard_archmage_button_lock.clone(),
    }
  }

  pub fn get_tile(&self, tile: &Tile) -> Handle<Image> {
    match tile {
      Tile::Grass => self.grass_tile.clone(),
      Tile::Water => self.water_tile.clone(),
      Tile::Path(_) | Tile::Spawn | Tile::End => self.path_tile.clone(),
      _ => self.grass_tile.clone(),
    }
  }
}

fn load_assets(
  mut commands: Commands,
  assets_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  commands.insert_resource(GameAssets {
    // Fonts
    font: assets_server.load("fonts/FiraSans-Bold.ttf"),

    // Main menu buttons - Start Game & Exit
    start_button: assets_server.load("textures/start_menu/start_button.png"),
    exit_button: assets_server.load("textures/start_menu/exit_button.png"),

    // Map tiles
    grass_tile: assets_server.load("textures/map/grass.png"),
    water_tile: assets_server.load("textures/map/water.png"),
    path_tile: assets_server.load("textures/map/path.png"),

    // Gameplay images - Health & Money
    heart: assets_server.load("textures/ui/heart.png"),
    coin: assets_server.load("textures/ui/coin.png"),

    // Towers
    wizard_nature: assets_server.load("textures/towers/wizard_nature.png"),
    wizard_fire: assets_server.load("textures/towers/wizard_fire.png"),
    wizard_ice: assets_server.load("textures/towers/wizard_ice.png"),
    wizard_dark: assets_server.load("textures/towers/wizard_dark.png"),
    wizard_mage: assets_server.load("textures/towers/wizard_mage.png"),
    wizard_archmage: assets_server.load("textures/towers/wizard_archmage.png"),

    // Tower buttons
    wizard_nature_button: assets_server
      .load("textures/tower_buttons/buttons/wizard_nature_button.png"),
    wizard_fire_button: assets_server.load("textures/tower_buttons/buttons/wizard_fire_button.png"),
    wizard_ice_button: assets_server.load("textures/tower_buttons/buttons/wizard_ice_button.png"),
    wizard_dark_button: assets_server.load("textures/tower_buttons/buttons/wizard_dark_button.png"),
    wizard_mage_button: assets_server.load("textures/tower_buttons/buttons/wizard_mage_button.png"),
    wizard_archmage_button: assets_server
      .load("textures/tower_buttons/buttons/wizard_archmage_button.png"),

    // Tower buttons hovered
    wizard_nature_button_hover: assets_server
      .load("textures/tower_buttons/buttons_hover/wizard_nature_button_hover.png"),
    wizard_fire_button_hover: assets_server
      .load("textures/tower_buttons/buttons_hover/wizard_fire_button_hover.png"),
    wizard_ice_button_hover: assets_server
      .load("textures/tower_buttons/buttons_hover/wizard_ice_button_hover.png"),
    wizard_dark_button_hover: assets_server
      .load("textures/tower_buttons/buttons_hover/wizard_dark_button_hover.png"),
    wizard_mage_button_hover: assets_server
      .load("textures/tower_buttons/buttons_hover/wizard_mage_button_hover.png"),
    wizard_archmage_button_hover: assets_server
      .load("textures/tower_buttons/buttons_hover/wizard_archmage_button_hover.png"),

    // Tower buttons pressed
    wizard_nature_button_press: assets_server
      .load("textures/tower_buttons/buttons_press/wizard_nature_button_press.png"),
    wizard_fire_button_press: assets_server
      .load("textures/tower_buttons/buttons_press/wizard_fire_button_press.png"),
    wizard_ice_button_press: assets_server
      .load("textures/tower_buttons/buttons_press/wizard_ice_button_press.png"),
    wizard_dark_button_press: assets_server
      .load("textures/tower_buttons/buttons_press/wizard_dark_button_press.png"),
    wizard_mage_button_press: assets_server
      .load("textures/tower_buttons/buttons_press/wizard_mage_button_press.png"),
    wizard_archmage_button_press: assets_server
      .load("textures/tower_buttons/buttons_press/wizard_archmage_button_press.png"),

    // Tower buttons locked
    wizard_nature_button_lock: assets_server
      .load("textures/tower_buttons/buttons_lock/wizard_nature_button_lock.png"),
    wizard_fire_button_lock: assets_server
      .load("textures/tower_buttons/buttons_lock/wizard_fire_button_lock.png"),
    wizard_ice_button_lock: assets_server
      .load("textures/tower_buttons/buttons_lock/wizard_ice_button_lock.png"),
    wizard_dark_button_lock: assets_server
      .load("textures/tower_buttons/buttons_lock/wizard_dark_button_lock.png"),
    wizard_mage_button_lock: assets_server
      .load("textures/tower_buttons/buttons_lock/wizard_mage_button_lock.png"),
    wizard_archmage_button_lock: assets_server
      .load("textures/tower_buttons/buttons_lock/wizard_archmage_button_lock.png"),

    // Selected tower UI - Tower icons
    wizard_nature_icon: assets_server
      .load("textures/selected_tower_ui/tower_icons/wizard_nature_icon.png"),
    wizard_fire_icon: assets_server
      .load("textures/selected_tower_ui/tower_icons/wizard_fire_icon.png"),
    wizard_ice_icon: assets_server
      .load("textures/selected_tower_ui/tower_icons/wizard_ice_icon.png"),
    wizard_dark_icon: assets_server
      .load("textures/selected_tower_ui/tower_icons/wizard_dark_icon.png"),
    wizard_mage_icon: assets_server
      .load("textures/selected_tower_ui/tower_icons/wizard_mage_icon.png"),
    wizard_archmage_icon: assets_server
      .load("textures/selected_tower_ui/tower_icons/wizard_archmage_icon.png"),

    // Selected tower UI - Targeting Priority buttons
    prev_target_button: assets_server
      .load("textures/selected_tower_ui/targeting_priority_ui/prev_button.png"),
    next_target_button: assets_server
      .load("textures/selected_tower_ui/targeting_priority_ui/next_button.png"),

    // Selected tower UI - Sell button
    sell_button: assets_server.load("textures/selected_tower_ui/sell_button/sell_button.png"),

    // Selected tower UI - Upgrades
    upgrade_button: assets_server.load("textures/selected_tower_ui/upgrades/upgrade_button.png"),
    upgrades: [
      assets_server.load("textures/selected_tower_ui/upgrades/upgrade0.png"),
      assets_server.load("textures/selected_tower_ui/upgrades/upgrade1.png"),
      assets_server.load("textures/selected_tower_ui/upgrades/upgrade2.png"),
      assets_server.load("textures/selected_tower_ui/upgrades/upgrade3.png"),
      assets_server.load("textures/selected_tower_ui/upgrades/upgrade4.png"),
      assets_server.load("textures/selected_tower_ui/upgrades/upgrade5.png"),
    ],

    // Enemies
    enemy: texture_atlases.add(TextureAtlas::from_grid(
      assets_server.load("textures/enemies/slime_jump.png"),
      Vec2::new(50., 90.),
      10,
      8,
      Some(Vec2::new(5., 5.)),
      None,
    )),
    // Bullets !!!
    wizard_nature_bullet: assets_server.load("textures/tower_bullets/wizard_nature_bullet.png"),
    wizard_fire_bullet: assets_server.load("textures/tower_bullets/wizard_fire_bullet.png"),
    wizard_ice_bullet: assets_server.load("textures/tower_bullets/wizard_ice_bullet.png"),
    wizard_dark_bullet: assets_server.load("textures/tower_bullets/wizard_dark_bullet.png"),
    wizard_mage_bullet: assets_server.load("textures/tower_bullets/wizard_mage_bullet.png"),
    wizard_archmage_bullet: assets_server.load("textures/tower_bullets/wizard_archmage_bullet.png"),
  })
}

// Slime sprite animation

#[derive(Component, Deref, DerefMut, Serialize, Deserialize, Debug, Clone)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct AnimationIndices {
  pub first: usize,
  pub last: usize,
}

fn animate_enemy_sprite(
  time: Res<Time>,
  mut query: Query<(
    &AnimationIndices,
    &mut AnimationTimer,
    &mut TextureAtlasSprite,
    &Movement,
  )>,
) {
  for (indices, mut timer, mut sprite, movement) in &mut query {
    // Change direction based on where enemy is heading
    if movement.direction.x != 0. {
      if movement.direction.x < 0. {
        sprite.flip_x = true;
      } else {
        sprite.flip_x = false;
      }
    }

    // Animate sprite
    timer.tick(time.delta());

    if timer.just_finished() {
      if sprite.index == indices.last {
        sprite.index = indices.first;
      } else {
        sprite.index += 1;
      }
    }
  }
}
