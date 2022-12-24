use bevy::prelude::*;

fn main() {
    App::new() // Make new app
    .add_plugins(DefaultPlugins) // Add basic game functionality - window, game tick, startup systems, etc.
    .run() // Execute app
}
