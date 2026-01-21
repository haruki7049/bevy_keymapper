use bevy::prelude::*;
use bevy_keymapper::{Keymap, Keymapper};

#[derive(PartialEq)]
enum KeymapLabel {
    Send,
}

#[derive(Debug, Resource)]
#[allow(dead_code)]
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
        .insert_resource(Keymapper::<KeymapLabel>::new(vec![Keymap::new(
            KeymapLabel::Send,
            KeyCode::Space,
            example_action_system,
        )]))
        .add_systems(Update, bevy_keymapper::keymaps_runner_system::<KeymapLabel>)
        .run();
}

fn example_action_system(stats: Res<PlayerStats>) {
    println!("Action triggered for player: {:?}", stats);
}
