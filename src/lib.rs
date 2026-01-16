//! # bevy_keymapper
//!
//! A flexible key mapping library for the Bevy game engine.
//!
//! `bevy_keymapper` provides a simple and extensible way to bind keyboard inputs to actions
//! in your Bevy applications. It allows you to define custom key mappings with access to
//! game state through the `Environment` trait.
//!
//! ## Features
//!
//! - Simple key-to-action mapping system
//! - Access to game state through the `Environment` trait
//! - Easy integration with Bevy's ECS
//! - Type-safe environment downcasting
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_keymapper::{Environment, Keymap, KeymapsManager, keymaps_runner_system};
//!
//! #[derive(Resource, Environment)]
//! struct GameState {
//!     score: i32,
//! }
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .insert_resource(GameState { score: 0 })
//!         .insert_resource(KeymapsManager {
//!             keymaps: vec![
//!                 Keymap {
//!                     keycode: KeyCode::Space,
//!                     function: Box::new(|_commands, env| {
//!                         if let Some(state) = env.as_any().downcast_ref::<GameState>() {
//!                             println!("Score: {}", state.score);
//!                         }
//!                     }),
//!                 },
//!             ],
//!         })
//!         .add_systems(Update, keymaps_runner_system::<GameState>)
//!         .run();
//! }
//! ```

pub use bevy_keymapper_derive::Environment;

use bevy::prelude::*;
use std::any::Any;

/// Manager for keyboard mappings.
///
/// This resource holds a collection of key mappings that will be processed
/// by the [`keymaps_runner_system`]. Each mapping associates a key code with
/// a function to execute when that key is pressed.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_keymapper::{KeymapsManager, Keymap};
///
/// let manager = KeymapsManager::new(vec![
///     Keymap {
///         keycode: KeyCode::Space,
///         function: Box::new(|_commands, _env| {
///             println!("Space pressed!");
///         }),
///     },
/// ]);
/// ```
#[derive(Resource)]
pub struct KeymapsManager {
    /// The collection of key mappings to be processed.
    pub keymaps: Vec<Keymap>,
}

impl KeymapsManager {
    /// Creates a new `KeymapsManager` with the given key mappings.
    ///
    /// # Arguments
    ///
    /// * `keymaps` - A vector of [`Keymap`] structs defining the key bindings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bevy::prelude::*;
    /// use bevy_keymapper::{KeymapsManager, Keymap};
    ///
    /// let manager = KeymapsManager::new(vec![]);
    /// ```
    pub fn new(keymaps: Vec<Keymap>) -> Self {
        Self { keymaps }
    }
}

/// A single key mapping that associates a key code with an action.
///
/// Each keymap defines what happens when a specific key is pressed. The function
/// receives mutable access to Bevy's [`Commands`] for spawning entities or running
/// commands, and read access to the environment resource for accessing game state.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_keymapper::Keymap;
///
/// let keymap = Keymap {
///     keycode: KeyCode::Space,
///     function: Box::new(|commands, env| {
///         // Your action here
///         println!("Key pressed!");
///     }),
/// };
/// ```
pub struct Keymap {
    /// The key code that triggers this mapping.
    pub keycode: KeyCode,
    /// The function to execute when the key is pressed.
    ///
    /// The function receives:
    /// - `&mut Commands` - Bevy commands for spawning entities or running deferred operations
    /// - `&dyn Environment` - Read-only access to the environment resource
    pub function: Box<dyn Fn(&mut Commands, &dyn Environment) + Send + Sync>,
}

/// Trait for environment types that can be used with the key mapping system.
///
/// This trait enables type-safe downcasting of the environment parameter in keymap
/// functions. Implement this trait for your game state resources to make them
/// accessible in key mapping functions.
///
/// The easiest way to implement this trait is by using the `#[derive(Environment)]`
/// macro provided by this crate.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_keymapper::Environment;
///
/// #[derive(Resource, Environment)]
/// struct GameState {
///     hp: i32,
///     name: String,
/// }
/// ```
pub trait Environment: Any + Send + Sync {
    /// Returns a reference to `self` as `&dyn Any`.
    ///
    /// This enables downcasting to the concrete type using `downcast_ref`.
    fn as_any(&self) -> &dyn Any;
    
    /// Returns a mutable reference to `self` as `&mut dyn Any`.
    ///
    /// This enables downcasting to the concrete type using `downcast_mut`.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// System that processes key mappings and executes associated actions.
///
/// This system should be added to the `Update` schedule. It checks for newly pressed
/// keys and executes the corresponding functions from the [`KeymapsManager`] resource.
///
/// # Type Parameters
///
/// * `E` - The environment type that implements [`Environment`] and [`Resource`].
///         This is the game state that will be accessible to keymap functions.
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_keymapper::{Environment, keymaps_runner_system};
///
/// #[derive(Resource, Environment)]
/// struct GameState {
///     score: i32,
/// }
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .insert_resource(GameState { score: 0 })
///         // Add the system with your environment type
///         .add_systems(Update, keymaps_runner_system::<GameState>)
///         .run();
/// }
/// ```
pub fn keymaps_runner_system<E: Environment + Resource>(
    mut commands: Commands,
    environment: Res<E>,
    manager: Res<KeymapsManager>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for &keycode in keyboard_input.get_just_pressed() {
        for keymap in manager.keymaps.iter() {
            if keymap.keycode == keycode {
                (keymap.function)(&mut commands, &*environment);
            }
        }
    }
}
