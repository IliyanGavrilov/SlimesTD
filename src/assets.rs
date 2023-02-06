use bevy::prelude::*;
use crate::Movement;

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

fn load_assets(
  mut commands: Commands,
  assets_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
  commands.insert_resource(GameAssets {
    // Towers
    wizard_nature: assets_server.load("towers/wizard_nature.png"),
    wizard_fire: assets_server.load("towers/wizard_fire.png"),
    wizard_ice: assets_server.load("towers/wizard_ice.png"),
    wizard_dark: assets_server.load("towers/wizard_dark.png"),
    wizard_mage: assets_server.load("towers/wizard_mage.png"),
    wizard_archmage: assets_server.load("towers/wizard_archmage.png"),
    // Tower buttons
    wizard_nature_button: assets_server.load("tower_buttons/wizard_nature_button.png"),
    wizard_fire_button: assets_server.load("tower_buttons/wizard_fire_button.png"),
    wizard_ice_button: assets_server.load("tower_buttons/wizard_ice_button.png"),
    wizard_dark_button: assets_server.load("tower_buttons/wizard_dark_button.png"),
    wizard_mage_button: assets_server.load("tower_buttons/wizard_mage_button.png"),
    wizard_archmage_button: assets_server.load("tower_buttons/wizard_archmage_button.png"),
    // Tower buttons hovered
    wizard_nature_button_hover: assets_server.load("tower_buttons/wizard_nature_button_hover.png"),
    wizard_fire_button_hover: assets_server.load("tower_buttons/wizard_fire_button_hover.png"),
    wizard_ice_button_hover: assets_server.load("tower_buttons/wizard_ice_button_hover.png"),
    wizard_dark_button_hover: assets_server.load("tower_buttons/wizard_dark_button_hover.png"),
    wizard_mage_button_hover: assets_server.load("tower_buttons/wizard_mage_button_hover.png"),
    wizard_archmage_button_hover: assets_server.load("tower_buttons/wizard_archmage_button_hover.png"),
    // Tower buttons pressed
    wizard_nature_button_press: assets_server.load("tower_buttons/wizard_nature_button_press.png"),
    wizard_fire_button_press: assets_server.load("tower_buttons/wizard_fire_button_press.png"),
    wizard_ice_button_press: assets_server.load("tower_buttons/wizard_ice_button_press.png"),
    wizard_dark_button_press: assets_server.load("tower_buttons/wizard_dark_button_press.png"),
    wizard_mage_button_press: assets_server.load("tower_buttons/wizard_mage_button_press.png"),
    wizard_archmage_button_press: assets_server.load("tower_buttons/wizard_archmage_button_press.png"),
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