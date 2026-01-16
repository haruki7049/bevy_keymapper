use bevy::prelude::*;
use bevy_keymapper::{KeymapperAppExt, keymaps_runner_system};

#[derive(PartialEq)]
enum KeymapLabel {
    Send,
}

#[derive(Debug, Resource)]
struct PlayerStats {
    hp: i32,
    name: String,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(PlayerStats {
            hp: 100,
            name: "Haruki".into(),
        })
        .add_keymap(KeymapLabel::Send, KeyCode::Space, example_action_system)
        .add_systems(Update, keymaps_runner_system::<KeymapLabel>)
        .run();
}

fn example_action_system(stats: Res<PlayerStats>) {
    println!("Action triggered for player: {:?}", stats);
}
