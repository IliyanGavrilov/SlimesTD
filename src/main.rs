#![windows_subsystem = "windows"] // Disable console
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod map;
pub use map::*;
mod main_menu;
pub use main_menu::*;
mod gameplay_ui;
pub use gameplay_ui::*;
mod assets;
pub use assets::*;
mod tower;
pub use tower::*;
mod enemy;
pub use enemy::*;
mod movement;
pub use movement::*;
mod game_data;
pub use game_data::*;

fn main() {
  App::new()
    // Background of window. Set colour of screen on each refresh
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    // Add basic game functionality - window, game tick, renderer,
    // asset loading, UI system, input, startup systems, etc.
    .add_plugins(
      DefaultPlugins
        // Prevent blurry sprites
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Slimes Tower Defense".to_string(),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resizable: false,
            ..default()
          }),
          ..default()
        }),
    )
    // Game State
    .add_state::<GameState>()
    // Asset loading
    .add_plugin(RonAssetPlugin::<EnemyTypeStats>::new(&["enemy_types.ron"]))
    .add_plugin(RonAssetPlugin::<Map>::new(&["map.ron"]))
    .add_plugin(RonAssetPlugin::<TowerTypeStats>::new(&["tower_stats.ron"]))
    .add_plugin(RonAssetPlugin::<Upgrades>::new(&["upgrades.ron"]))
    .add_plugin(RonAssetPlugin::<Waves>::new(&["waves.ron"]))
    .add_loading_state(
      LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::MainMenu),
    )
    .add_collection_to_loading_state::<_, GameData>(GameState::AssetLoading)
    // Plugins
    .add_plugin(MainMenuPlugin)
    .add_plugin(GameplayUIPlugin)
    .add_plugin(MapPlugin)
    //.add_plugin(SpawnScenePlugin)
    .add_plugin(SettingsPlugin)
    .add_plugin(AssetPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(BasePlugin)
    .add_plugin(TowerPlugin)
    .add_plugin(TowerButtonPlugin)
    .add_plugin(TowerSelectionPlugin)
    .add_plugin(TowerUIPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(WavePlugin)
    .add_plugin(BulletPlugin)
    .add_plugin(MovementPlugin)
    // !!!Debugging
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .run();
}
