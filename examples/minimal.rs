use bevy::prelude::*;
use bevy_keymapper::{Environment, Keymap, KeymapsManager, keymaps_runner_system};
use std::any::Any;

#[derive(Resource, Environment)]
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
                function: Box::new(|_commands, env| {
                    // Downcast to the specific implementation
                    if let Some(stats) = env.as_any().downcast_ref::<PlayerStats>() {
                        println!(
                            "Player '{}' (HP: {}) triggered an action!",
                            stats.name, stats.hp
                        );
                    }
                }),
            }],
        })
        // 3. Register the system with the specific Environment type
        .add_systems(Update, keymaps_runner_system::<PlayerStats>)
        .run();
}
