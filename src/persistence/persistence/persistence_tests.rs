use super::*;

#[test]
fn vsync_on_uses_auto_vsync() {
  assert_eq!(present_mode(true), PresentMode::AutoVsync);
}

#[test]
fn vsync_off_uses_auto_no_vsync() {
  assert_eq!(present_mode(false), PresentMode::AutoNoVsync);
}

#[test]
fn fullscreen_on_uses_borderless() {
  assert_eq!(window_mode(true), WindowMode::BorderlessFullscreen);
}

#[test]
fn fullscreen_off_uses_windowed() {
  assert_eq!(window_mode(false), WindowMode::Windowed);
}

#[test]
fn default_video_settings_enable_vsync() {
  assert!(VideoSettings::default().vsync);
}

#[test]
fn default_video_settings_disable_fullscreen() {
  assert!(!VideoSettings::default().fullscreen);
}
