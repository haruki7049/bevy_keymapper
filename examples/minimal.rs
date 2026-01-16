use bevy::prelude::*;
use bevy_keymapper::{Keymap, KeymapsManager, keymaps_runner_system};

#[derive(Debug, Resource)]
struct PlayerStats {
    hp: i32,
    name: String,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 1. Setup the environment resource
        .insert_resource(PlayerStats {
            hp: 100,
            name: "Haruki".to_string(),
        })
        // 2. Setup the keymaps
        .insert_resource(KeymapsManager {
            keymaps: vec![Keymap {
                keycode: KeyCode::Space,
                system: Box::new(IntoSystem::into_system(example_action_system)),
            }],
        })
        // 3. Register the system with the specific Environment type
        .add_systems(Update, keymaps_runner_system)
        .run();
}

fn example_action_system(stats: Res<PlayerStats>) {
    println!("Action triggered for player: {:?}", stats);
}
