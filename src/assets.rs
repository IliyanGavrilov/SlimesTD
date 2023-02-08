use bevy::prelude::*;
use crate::{Movement, TowerType};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(animate_enemy_sprite)
       // Load assets before the startup stage, so we can use them in spawn_basic_scene()
       .add_startup_system_to_stage(StartupStage::PreStartup, load_assets);
  }
}

// Asset loading

#[derive(Resource)]
pub struct GameAssets {
  // Main menu buttons - Start Game & Exit
  pub start_button: Handle<Image>,
  pub exit_button: Handle<Image>,
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
  // Enemies
  pub enemy: Handle<TextureAtlas>, // !!!! Rename
  // Tower bullets
  pub wizard_nature_bullet: Handle<Image>,
  pub wizard_fire_bullet: Handle<Image>,
  pub wizard_ice_bullet: Handle<Image>,
  pub wizard_dark_bullet: Handle<Image>,
  pub wizard_mage_bullet: Handle<Image>,
  pub wizard_archmage_bullet: Handle<Image>
}
impl GameAssets {
  pub fn get_tower_asset(&self, tower_type: TowerType) -> Handle<Image> {
    match tower_type {
      TowerType::Nature => self.wizard_nature.clone(),
      TowerType::Fire => self.wizard_fire.clone(),
      TowerType::Ice => self.wizard_ice.clone(),
      TowerType::Dark => self.wizard_dark.clone(),
      TowerType::Mage => self.wizard_mage.clone(),
      TowerType::Archmage => self.wizard_archmage.clone()
    }
  }
  
  pub fn get_button_asset(&self, tower_type: TowerType) -> Handle<Image> {
    match tower_type {
      TowerType::Nature => self.wizard_nature_button.clone(),
      TowerType::Fire => self.wizard_fire_button.clone(),
      TowerType::Ice => self.wizard_ice_button.clone(),
      TowerType::Dark => self.wizard_dark_button.clone(),
      TowerType::Mage => self.wizard_mage_button.clone(),
      TowerType::Archmage => self.wizard_archmage_button.clone()
    }
  }
  
  pub fn get_button_hovered_asset(&self, tower_type: TowerType) -> Handle<Image> {
    match tower_type {
      TowerType::Nature => self.wizard_nature_button_hover.clone(),
      TowerType::Fire => self.wizard_fire_button_hover.clone(),
      TowerType::Ice => self.wizard_ice_button_hover.clone(),
      TowerType::Dark => self.wizard_dark_button_hover.clone(),
      TowerType::Mage => self.wizard_mage_button_hover.clone(),
      TowerType::Archmage => self.wizard_archmage_button_hover.clone()
    }
  }
  
  pub fn get_button_pressed_asset(&self, tower_type: TowerType) -> Handle<Image> {
    match tower_type {
      TowerType::Nature => self.wizard_nature_button_press.clone(),
      TowerType::Fire => self.wizard_fire_button_press.clone(),
      TowerType::Ice => self.wizard_ice_button_press.clone(),
      TowerType::Dark => self.wizard_dark_button_press.clone(),
      TowerType::Mage => self.wizard_mage_button_press.clone(),
      TowerType::Archmage => self.wizard_archmage_button_press.clone()
    }
  }
}

fn load_assets(
  mut commands: Commands,
  assets_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
  commands.insert_resource(GameAssets {
    //
    start_button: assets_server.load("start_menu/start_button.png"),
    exit_button: assets_server.load("start_menu/exit_button.png"),
    // Towers
    wizard_nature: assets_server.load("towers/wizard_nature.png"),
    wizard_fire: assets_server.load("towers/wizard_fire.png"),
    wizard_ice: assets_server.load("towers/wizard_ice.png"),
    wizard_dark: assets_server.load("towers/wizard_dark.png"),
    wizard_mage: assets_server.load("towers/wizard_mage.png"),
    wizard_archmage: assets_server.load("towers/wizard_archmage.png"),
    // Tower buttons
    wizard_nature_button: assets_server.load("tower_buttons/buttons/wizard_nature_button.png"),
    wizard_fire_button: assets_server.load("tower_buttons/buttons/wizard_fire_button.png"),
    wizard_ice_button: assets_server.load("tower_buttons/buttons/wizard_ice_button.png"),
    wizard_dark_button: assets_server.load("tower_buttons/buttons/wizard_dark_button.png"),
    wizard_mage_button: assets_server.load("tower_buttons/buttons/wizard_mage_button.png"),
    wizard_archmage_button: assets_server.load("tower_buttons/buttons/wizard_archmage_button.png"),
    // Tower buttons hovered
    wizard_nature_button_hover: assets_server.load("tower_buttons/buttons_hover/wizard_nature_button_hover.png"),
    wizard_fire_button_hover: assets_server.load("tower_buttons/buttons_hover/wizard_fire_button_hover.png"),
    wizard_ice_button_hover: assets_server.load("tower_buttons/buttons_hover/wizard_ice_button_hover.png"),
    wizard_dark_button_hover: assets_server.load("tower_buttons/buttons_hover/wizard_dark_button_hover.png"),
    wizard_mage_button_hover: assets_server.load("tower_buttons/buttons_hover/wizard_mage_button_hover.png"),
    wizard_archmage_button_hover: assets_server.load("tower_buttons/buttons_hover/wizard_archmage_button_hover.png"),
    // Tower buttons pressed
    wizard_nature_button_press: assets_server.load("tower_buttons/buttons_press/wizard_nature_button_press.png"),
    wizard_fire_button_press: assets_server.load("tower_buttons/buttons_press/wizard_fire_button_press.png"),
    wizard_ice_button_press: assets_server.load("tower_buttons/buttons_press/wizard_ice_button_press.png"),
    wizard_dark_button_press: assets_server.load("tower_buttons/buttons_press/wizard_dark_button_press.png"),
    wizard_mage_button_press: assets_server.load("tower_buttons/buttons_press/wizard_mage_button_press.png"),
    wizard_archmage_button_press: assets_server.load("tower_buttons/buttons_press/wizard_archmage_button_press.png"),
    // Enemies
    enemy: texture_atlases.add(
      TextureAtlas::from_grid(assets_server.load("slime_jump.png"),
                              Vec2::new(50., 90.),
                              10, 8,
                              Some(Vec2::new(5., 5.)), None)),
    // Bullets !!!!!
    wizard_nature_bullet: assets_server.load("tower_bullets/wizard_nature_bullet.png"),
    wizard_fire_bullet: assets_server.load("tower_bullets/wizard_fire_bullet.png"),
    wizard_ice_bullet: assets_server.load("tower_bullets/wizard_ice_bullet.png"),
    wizard_dark_bullet: assets_server.load("tower_bullets/wizard_dark_bullet.png"),
    wizard_mage_bullet: assets_server.load("tower_bullets/wizard_mage_bullet.png"),
    wizard_archmage_bullet: assets_server.load("tower_bullets/wizard_archmage_bullet.png")
  })
}



// Slime sprite animation

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer); // !!!

#[derive(Component)]
pub struct AnimationIndices {
  pub first: usize, // !!!
  pub last: usize // !!!
}

fn animate_enemy_sprite(
  time: Res<Time>,
  mut query: Query<(
    &AnimationIndices,
    &mut AnimationTimer,
    &mut TextureAtlasSprite,
    &GlobalTransform,
    &Movement
  )>
) {
  for (indices,
    mut timer,
    mut sprite,
    transform,
    movement) in &mut query {
    // Change direction based on where enemy is heading
    if transform.translation().x > movement.direction.x {
      sprite.flip_x = true;
    }
    else {
      sprite.flip_x = false;
    }
    
    // Animate sprite
    timer.tick(time.delta());
    
    if timer.just_finished() {
      if sprite.index == indices.last {
        sprite.index = indices.first;
      }
      else {
        sprite.index += 1;
      }
    }
  }
}