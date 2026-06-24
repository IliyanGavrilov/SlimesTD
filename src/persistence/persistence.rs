use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::AudioSettings;

/// Centralised, platform-aware persistence for user settings and progress.
///
/// - Settings (volume + video) live in `settings.ron` and are portable.
/// - Progress (tutorial flag, future unlocks/scores) lives in `progress.ron`.
///
/// Storage location (see the `backend` module):
/// - Native **release** build  -> OS config dir, e.g. `%APPDATA%/SlimesTD/`.
/// - Native **dev** build       -> the working directory (so test runs don't
///   pollute your real profile, and the files are easy to delete/inspect).
/// - **Web** (wasm/itch.io)     -> the browser's `localStorage`.
pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
  fn build(&self, app: &mut App) {
    let settings: SettingsFile = load_or_default(SETTINGS_KEY);

    app
      .insert_resource(settings.audio)
      .insert_resource(settings.video)
      // Push saved video settings onto the window once it exists.
      .add_startup_system(apply_video_settings)
      // Keep VideoSettings in sync with the live window (toggles mutate the
      // window directly), then persist whenever anything changed.
      .add_system(sync_video_settings)
      .add_system(persist_settings.after(sync_video_settings));
  }
}

const SETTINGS_KEY: &str = "settings";

/// Combined on-disk settings document. Resources stay split in the ECS
/// (`AudioSettings`, `VideoSettings`); this struct only exists for (de)serialising
/// them as one file.
#[derive(Serialize, Deserialize, Default)]
struct SettingsFile {
  #[serde(default)]
  audio: AudioSettings,
  #[serde(default)]
  video: VideoSettings,
}

/// Persisted display preferences, applied to the primary window on launch.
#[derive(Resource, Serialize, Deserialize, Clone, PartialEq)]
pub struct VideoSettings {
  pub vsync: bool,
  pub fullscreen: bool,
}

impl Default for VideoSettings {
  fn default() -> Self {
    Self {
      vsync: true,
      fullscreen: false,
    }
  }
}

fn apply_video_settings(video: Res<VideoSettings>, mut windows: Query<&mut Window>) {
  if let Ok(mut window) = windows.get_single_mut() {
    window.present_mode = present_mode(video.vsync);
    window.mode = window_mode(video.fullscreen);
  }
}

/// The window is the source of truth (V / Alt+Enter shortcuts and the Settings
/// toggles all mutate it). Mirror its state back into `VideoSettings` so it can
/// be saved; the inner guard means the resource is only marked changed on a real
/// change, which keeps `persist_settings` from writing every frame.
fn sync_video_settings(windows: Query<&Window>, mut video: ResMut<VideoSettings>) {
  if let Ok(window) = windows.get_single() {
    let vsync = matches!(window.present_mode, PresentMode::AutoVsync);
    let fullscreen = !matches!(window.mode, WindowMode::Windowed);
    if video.vsync != vsync || video.fullscreen != fullscreen {
      video.vsync = vsync;
      video.fullscreen = fullscreen;
    }
  }
}

fn persist_settings(
  audio: Res<AudioSettings>,
  video: Res<VideoSettings>,
  mut initialised: Local<bool>,
) {
  // Skip the first frame: both resources read as "changed" right after insertion,
  // and we don't want to rewrite the file on every launch when nothing changed.
  if !*initialised {
    *initialised = true;
    return;
  }

  if audio.is_changed() || video.is_changed() {
    save(
      SETTINGS_KEY,
      &SettingsFile {
        audio: audio.clone(),
        video: video.clone(),
      },
    );
  }
}

fn present_mode(vsync: bool) -> PresentMode {
  if vsync {
    PresentMode::AutoVsync
  } else {
    PresentMode::AutoNoVsync
  }
}

fn window_mode(fullscreen: bool) -> WindowMode {
  if fullscreen {
    WindowMode::BorderlessFullscreen
  } else {
    WindowMode::Windowed
  }
}

/// Reads `<key>.ron`, falling back to `T::default()` if it's missing or invalid.
pub fn load_or_default<T: DeserializeOwned + Default>(key: &str) -> T {
  backend::read(key)
    .and_then(|raw| ron::from_str(&raw).ok())
    .unwrap_or_default()
}

/// Best-effort save; IO/serialisation errors are swallowed (never worth crashing
/// the game over a settings write).
pub fn save<T: Serialize>(key: &str, value: &T) {
  if let Ok(text) = ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default()) {
    backend::write(key, &text);
  }
}

// --- Platform backends -------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
mod backend {
  use std::path::PathBuf;

  fn dir() -> PathBuf {
    // Dev builds keep saves next to the executable's working dir for easy
    // inspection; release builds use the per-user OS config directory.
    #[cfg(debug_assertions)]
    {
      PathBuf::from(".")
    }
    #[cfg(not(debug_assertions))]
    {
      let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
      path.push("SlimesTD");
      path
    }
  }

  fn path(key: &str) -> PathBuf {
    dir().join(format!("{key}.ron"))
  }

  pub fn read(key: &str) -> Option<String> {
    std::fs::read_to_string(path(key)).ok()
  }

  pub fn write(key: &str, text: &str) {
    let _ = std::fs::create_dir_all(dir());
    let _ = std::fs::write(path(key), text);
  }
}

#[cfg(target_arch = "wasm32")]
mod backend {
  // Browsers (itch.io) have no filesystem, so persist to localStorage instead.
  fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
  }

  fn storage_key(key: &str) -> String {
    format!("slimestd.{key}")
  }

  pub fn read(key: &str) -> Option<String> {
    storage()?.get_item(&storage_key(key)).ok()?
  }

  pub fn write(key: &str, text: &str) {
    if let Some(storage) = storage() {
      let _ = storage.set_item(&storage_key(key), text);
    }
  }
}

#[cfg(test)]
#[path = "persistence/persistence_tests.rs"]
mod tests;
