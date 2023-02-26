use bevy::prelude::*;
use bevy_editor_pls::*;

mod map;
pub use map::*;
mod main_menu;
pub use main_menu::*;
mod gameplay_ui;
pub use gameplay_ui::*;
mod spawn_scene;
pub use spawn_scene::*;
mod assets;
pub use assets::*;
mod tower;
pub use tower::*;
mod enemy;
pub use enemy::*;
mod movement;
pub use movement::*;

fn main() {
  App::new()
    // Background of window. Set colour of screen on each refresh
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
  
    // Add basic game functionality - window, game tick, renderer,
    // asset loading, UI system, input, startup systems, etc.
    .add_plugins(DefaultPlugins
      // Prevent blurry sprites
      .set(ImagePlugin::default_nearest())
      .set(WindowPlugin {
        window: WindowDescriptor {
          title: "Slimes Tower Defense".to_string(),
          position: WindowPosition::Centered,
          //resizable: false,
          ..default()
        },
        ..default()
      }))
    
    // Game State
    .add_state(GameState::MainMenu)
    
    // Plugins
    .add_plugin(MainMenuPlugin)
    .add_plugin(GameplayUIPlugin)
    .add_plugin(MapPlugin) // !
    //.add_plugin(SpawnScenePlugin)
    .add_plugin(SettingsPlugin)
    .add_plugin(AssetPlugin) // ?
    .add_plugin(PlayerPlugin)
    .add_plugin(BasePlugin)
    .add_plugin(TowerPlugin)
    .add_plugin(TowerButtonPlugin)
    .add_plugin(TowerSelectionPlugin)
    .add_plugin(TowerUpgradePlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(WavePlugin)
    .add_plugin(BulletPlugin)
    .add_plugin(MovementPlugin)
    
    // !!!Debugging
    .add_plugin(EditorPlugin) // Similar to WorldInspectorPlugin
    .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .run();
}