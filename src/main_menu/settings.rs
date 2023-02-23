use bevy::{prelude::*, window::PresentMode};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(toggle_vsync)
       .add_system(toggle_fullscreen);
  }
}

fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
  if input.just_pressed(KeyCode::V) {
    let window = windows.primary_mut();
    
    window.set_present_mode(if matches!(window.present_mode(), PresentMode::AutoVsync) {
      PresentMode::AutoNoVsync
    } else {
      PresentMode::AutoVsync
    });
    info!("PRESENT_MODE: {:?}", window.present_mode());
  }
}

fn toggle_fullscreen(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
  if ((input.pressed(KeyCode::LAlt) || input.pressed(KeyCode::RAlt))
    && input.just_pressed(KeyCode::Return)) || input.just_pressed(KeyCode::F11) {
    let window = windows.primary_mut();
    
    window.set_mode (if matches!(window.mode(), WindowMode::Windowed) {
      WindowMode::BorderlessFullscreen
    } else {
      WindowMode::Windowed
    });
    info!("WINDOW_MODE: {:?}", window.mode());
  }
}