use bevy::prelude::*;
use bevy_keymapper::{KeymapperAppExt, keymaps_runner_system};

#[derive(PartialEq)]
enum DebugKeymaps {
    Send,
}

#[derive(PartialEq)]
enum ReleaseKeymaps {
    Send,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameMode {
    #[default]
    Debug,
    Release,
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
        .init_state::<GameMode>()
        .insert_resource(PlayerStats {
            hp: 100,
            name: "Haruki".into(),
        })
        .add_keymap(DebugKeymaps::Send, KeyCode::Space, debug_system)
        .add_keymap(ReleaseKeymaps::Send, KeyCode::Space, release_system)
        .add_systems(
            Update,
            keymaps_runner_system::<DebugKeymaps>.run_if(in_state(GameMode::Debug)),
        )
        .add_systems(
            Update,
            keymaps_runner_system::<ReleaseKeymaps>.run_if(in_state(GameMode::Release)),
        )
        .run();
}

fn debug_system(stats: Res<PlayerStats>) {
    println!("Action triggered for player: {:?}", stats);
}

fn release_system(stats: Res<PlayerStats>) {
    println!("Action triggered for player: {:?}", stats);
    println!("This is Release mode!!");
}
