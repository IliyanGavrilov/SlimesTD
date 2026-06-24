#![cfg_attr(not(test), windows_subsystem = "windows")]
// Disable console
// These lints fight Bevy's idioms rather than catch real problems: system signatures
// are inherently large (many queries/resources), and the flat re-export module pattern
// (`mod foo; pub use foo::*;` over `foo/foo.rs`) intentionally nests same-named modules.
// Allowed crate-wide instead of scattering per-item attributes.
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
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
mod audio;
pub use audio::*;
mod effects;
pub use effects::*;
mod tutorial;
pub use tutorial::*;
mod persistence;
pub use persistence::*;

fn main() {
  App::new()
    // Window clear colour.
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    // Window, rendering, input, asset loading, UI - the engine essentials.
    .add_plugins(
      DefaultPlugins
        // Prevent blurry sprites
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Slimes Tower Defense".to_string(),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resizable: false,
            // On web, track the parent element's size so the canvas fills the
            // itch.io / fullscreen container instead of staying a fixed size.
            fit_canvas_to_parent: true,
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
    .add_plugin(MapSelectionPlugin)
    .add_plugin(GameOverPlugin)
    .add_plugin(VictoryPlugin)
    .add_plugin(GameplayUIPlugin)
    .add_plugin(MapPlugin)
    .add_plugin(SettingsPlugin)
    .add_plugin(AssetPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(BasePlugin)
    .add_plugin(TowerPlugin)
    .add_plugin(FarmPlugin)
    .add_plugin(TowerButtonPlugin)
    .add_plugin(TowerSelectionPlugin)
    .add_plugin(TowerUIPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(WavePlugin)
    .add_plugin(BulletPlugin)
    .add_plugin(MovementPlugin)
    .add_plugin(GameAudioPlugin)
    .add_plugin(EffectsPlugin)
    .add_plugin(TutorialPlugin)
    .add_plugin(PersistencePlugin)
    // Debug-only tools
    .add_plugin(WorldInspectorPlugin::new())
    .run();
}
