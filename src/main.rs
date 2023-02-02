use bevy::prelude::*;
// !!!Debugging
use bevy_editor_pls::*;

mod spawn_scene;
pub use spawn_scene::*;
mod settings;
pub use settings::*;
mod assets;
pub use assets::*;
mod base;
pub use base::*;
mod tower;
pub use tower::*;
mod enemy;
pub use enemy::*;
mod bullet;
pub use bullet::*;
mod movement;
pub use movement::*;
mod targeting_priority;

// Background of window. The colour of the screen on each refresh
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

// Temp!!!
fn load(time: Res<Time>) {
  Timer::from_seconds(5., TimerMode::Once).tick(time.delta());
}

fn main() {
  App::new()
    // Background of window. Set colour of screen on each refresh
    .insert_resource(ClearColor(CLEAR))
  
    .add_startup_system_to_stage(StartupStage::PreStartup, load)
    
    .add_plugin(SpawnScenePlugin)
    .add_plugin(SettingsPlugin)
    .add_plugin(AssetPlugin)
    .add_plugin(TowerPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(BulletPlugin)
    .add_plugin(MovementPlugin)
    
    // Add basic game functionality - window, game tick, renderer,
    // asset loading, UI system, input, startup systems, etc.
    .add_plugins(DefaultPlugins
      .set(ImagePlugin::default_nearest()) // Prevent blurry sprites
      .set(WindowPlugin {
      window: WindowDescriptor {
        title: "Slimes Tower Defence".to_string(),
        ..default()
      },
      ..default()
    }))
    
    // !!!Debugging
    .add_plugin(EditorPlugin) // Similar to WorldInspectorPlugin
    .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .run();
}