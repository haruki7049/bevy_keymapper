# bevy_keymapper

[![Crates.io](https://img.shields.io/crates/v/bevy_keymapper.svg)](https://crates.io/crates/bevy_keymapper)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](./LICENSE-MIT)

A flexible and type-safe key mapping library for the [Bevy](https://bevyengine.org/) game engine.

## ⚠️ Work in Progress

This library is currently under active development. APIs may change without notice until version 1.0.

## Features

- 🎮 **Simple Key Mapping**: Easily bind keyboard inputs to actions
- 🔒 **Type-Safe**: Leverage Rust's type system for safe environment access
- 🎯 **ECS Integration**: Seamlessly integrates with Bevy's Entity Component System
- 🧩 **Flexible**: Access game state through the `Environment` trait
- 📦 **Lightweight**: Minimal dependencies, focused on core functionality

## Installation

Add `bevy_keymapper` to your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.17"
bevy_keymapper = "0.1"
```

## Quick Start

Here's a minimal example to get you started:

```rust
use bevy::prelude::*;
use bevy_keymapper::{Environment, Keymap, KeymapsManager, keymaps_runner_system};

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
```

## Usage Guide

### 1. Define Your Environment

Your environment is the game state that will be accessible to keymap functions. It must implement the `Environment` trait, which is easily done using the derive macro:

```rust
use bevy::prelude::*;
use bevy_keymapper::Environment;

#[derive(Resource, Environment)]
struct GameState {
    score: i32,
    level: u32,
    player_name: String,
}
```

### 2. Create Key Mappings

Define what happens when specific keys are pressed:

```rust
use bevy::prelude::*;
use bevy_keymapper::{Keymap, KeymapsManager};

fn setup_keymaps() -> KeymapsManager {
    KeymapsManager::new(vec![
        Keymap {
            keycode: KeyCode::Space,
            function: Box::new(|commands, env| {
                if let Some(state) = env.as_any().downcast_ref::<GameState>() {
                    println!("Score: {}", state.score);
                }
            }),
        },
        Keymap {
            keycode: KeyCode::KeyR,
            function: Box::new(|commands, env| {
                // Reset game or perform other actions
                println!("Reset triggered!");
            }),
        },
    ])
}
```

### 3. Register the System

Add the keymap runner system to your Bevy app:

```rust
use bevy::prelude::*;
use bevy_keymapper::keymaps_runner_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState {
            score: 0,
            level: 1,
            player_name: "Player".to_string(),
        })
        .insert_resource(setup_keymaps())
        .add_systems(Update, keymaps_runner_system::<GameState>)
        .run();
}
```

## Advanced Usage

### Spawning Entities

You can use the `Commands` parameter to spawn entities or perform other ECS operations:

```rust
Keymap {
    keycode: KeyCode::KeyE,
    function: Box::new(|commands, env| {
        commands.spawn((
            Transform::default(),
            Visibility::default(),
            // Other components...
        ));
    }),
}
```

### Multiple Key Mappings

Organize complex key mappings by building them programmatically:

```rust
fn create_keymaps() -> Vec<Keymap> {
    let mut keymaps = Vec::new();
    
    // Movement keys
    keymaps.push(Keymap {
        keycode: KeyCode::KeyW,
        function: Box::new(|_, _| println!("Move forward")),
    });
    
    keymaps.push(Keymap {
        keycode: KeyCode::KeyS,
        function: Box::new(|_, _| println!("Move backward")),
    });
    
    // Action keys
    keymaps.push(Keymap {
        keycode: KeyCode::Space,
        function: Box::new(|_, _| println!("Jump")),
    });
    
    keymaps
}
```

## Examples

Check out the [examples](./examples) directory for more complete examples:

- [`minimal.rs`](./examples/minimal.rs) - Basic usage demonstration

Run an example with:

```bash
cargo run --example minimal
```

## Bevy Compatibility

| bevy_keymapper | Bevy |
|----------------|------|
| 0.1            | 0.17 |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgments

Built with ❤️ for the Bevy game engine community.
