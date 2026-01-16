//! # bevy_keymapper
//!
//! A key mapper library for the Bevy game engine that allows you to easily bind keyboard inputs to systems.
//!
//! ## Overview
//!
//! This crate provides a simple and flexible way to map keyboard keys to Bevy systems. You can:
//! - Bind multiple systems to different keys
//! - Use custom labels to organize and manage keymaps
//! - Dynamically add or remove key bindings at runtime
//!
//! ## Example
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_keymapper::{KeymapperAppExt, keymaps_runner_system};
//!
//! #[derive(PartialEq)]
//! enum KeymapLabel {
//!     Jump,
//! }
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_keymap(KeymapLabel::Jump, KeyCode::Space, jump_system)
//!         .add_systems(Update, keymaps_runner_system::<KeymapLabel>)
//!         .run();
//! }
//!
//! fn jump_system() {
//!     println!("Jump!");
//! }
//! ```

use bevy::prelude::*;

/// A resource that manages a collection of keymaps.
///
/// This struct stores and manages multiple key bindings, each associated with a label of type `T`.
/// The manager is responsible for executing the appropriate systems when their corresponding keys are pressed.
///
/// # Type Parameters
///
/// * `T` - A label type used to identify and organize keymaps. Must implement `Send + Sync + 'static`.
#[derive(Resource)]
pub struct Keymapper<T: Send + Sync + 'static> {
    /// The collection of keymaps managed by this manager.
    pub keymaps: Vec<Keymap<T>>,
}

impl<T: PartialEq + Send + Sync + 'static> Keymapper<T> {
    /// Removes all keymaps with the specified label.
    ///
    /// # Arguments
    ///
    /// * `label` - The label of the keymaps to remove.
    ///
    /// # Example
    ///
    /// ```ignore
    /// manager.remove(KeymapLabel::Jump);
    /// ```
    pub fn remove(&mut self, label: T) {
        self.keymaps.retain(|k| k.label != label);
    }

    /// Creates a new `Keymapper` with the given keymaps.
    ///
    /// # Arguments
    ///
    /// * `keymaps` - A vector of keymaps to manage.
    ///
    /// # Returns
    ///
    /// A new `Keymapper` instance.
    pub fn new(keymaps: Vec<Keymap<T>>) -> Self {
        Self { keymaps }
    }

    /// Executes all systems associated with the specified keycode.
    ///
    /// This method iterates through all keymaps and runs the systems for those
    /// that match the given keycode. Systems are initialized on their first execution.
    ///
    /// # Arguments
    ///
    /// * `world` - The Bevy world to execute systems in.
    /// * `keycode` - The keycode that was pressed.
    ///
    /// # Returns
    ///
    /// `Ok(())` if all systems executed successfully, or an error if any system failed.
    ///
    /// # Errors
    ///
    /// Returns a `RunSystemError` if any system execution fails.
    pub fn run(
        &mut self,
        world: &mut World,
        keycode: KeyCode,
    ) -> Result<(), Box<bevy::ecs::system::RunSystemError>> {
        for keymap in &mut self.keymaps {
            if keymap.keycode == keycode {
                if !keymap.initialized {
                    keymap.system.initialize(world);
                    keymap.initialized = true;
                }

                keymap.system.run((), world)?;
                keymap.system.apply_deferred(world);
            }
        }

        Ok(())
    }
}

/// A mapping between a keyboard key and a Bevy system.
///
/// Each keymap associates a label, a keycode, and a system that should be executed
/// when the key is pressed. The system is lazily initialized on first execution.
///
/// # Type Parameters
///
/// * `T` - A label type used to identify this keymap.
pub struct Keymap<T> {
    /// The label identifying this keymap.
    pub label: T,
    /// The keyboard key that triggers this keymap.
    pub keycode: KeyCode,
    /// The system to execute when the key is pressed.
    pub system: Box<dyn System<In = (), Out = ()>>,
    /// Whether the system has been initialized.
    initialized: bool,
}

impl<T> Keymap<T> {
    /// Creates a new keymap binding a label and keycode to a system.
    ///
    /// # Arguments
    ///
    /// * `label` - A label to identify this keymap.
    /// * `keycode` - The keyboard key that will trigger the system.
    /// * `system` - The system to execute when the key is pressed. This can be any function
    ///   that implements `IntoSystem<(), (), M>`.
    ///
    /// # Returns
    ///
    /// A new `Keymap` instance.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let keymap = Keymap::new(KeymapLabel::Jump, KeyCode::Space, jump_system);
    /// ```
    pub fn new<M>(label: T, keycode: KeyCode, system: impl IntoSystem<(), (), M>) -> Self {
        Self {
            label,
            keycode,
            system: Box::new(IntoSystem::into_system(system)),
            initialized: false,
        }
    }
}

/// A system that runs all keymaps for just-pressed keys.
///
/// This system should be added to your Bevy app to enable keymap functionality.
/// It checks for keyboard input every frame and executes the systems associated
/// with any keys that were just pressed.
///
/// # Type Parameters
///
/// * `T` - The label type used by your keymaps. Must implement `Send + Sync + PartialEq + 'static`.
///
/// # Example
///
/// ```ignore
/// app.add_systems(Update, keymaps_runner_system::<MyKeymapLabel>);
/// ```
///
/// # Errors
///
/// If a keymap system fails to execute, an error is logged but the runner continues processing
/// other keymaps.
pub fn keymaps_runner_system<T>(world: &mut World)
where
    T: Send + Sync + PartialEq + 'static,
{
    let keyboard_input = world.resource::<ButtonInput<KeyCode>>().clone();
    let keycodes: Vec<KeyCode> = keyboard_input.get_just_pressed().copied().collect();

    let result = world.resource_scope(
        |world,
         mut manager: Mut<Keymapper<T>>|
         -> Result<(), Box<bevy::ecs::system::RunSystemError>> {
            for keycode in keycodes {
                manager.run(world, keycode)?;
            }

            Ok(())
        },
    );

    if let Err(e) = result {
        error!("Keymapper error: {}", e);
    }
}

/// An extension trait for `App` that adds keymap functionality.
///
/// This trait provides convenience methods for adding keymaps to a Bevy application.
/// It automatically manages the `Keymapper` resource.
pub trait KeymapperAppExt {
    /// Adds a keymap binding to the application.
    ///
    /// This method binds a keyboard key to a system. When the key is pressed,
    /// the system will be executed. The `Keymapper` resource is automatically
    /// created if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `label` - A label to identify this keymap. Can be used later to remove the keymap.
    /// * `keycode` - The keyboard key that will trigger the system.
    /// * `system` - The system to execute when the key is pressed.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `App` for method chaining.
    ///
    /// # Example
    ///
    /// ```ignore
    /// app.add_keymap(KeymapLabel::Jump, KeyCode::Space, jump_system)
    ///    .add_keymap(KeymapLabel::Shoot, KeyCode::KeyX, shoot_system);
    /// ```
    fn add_keymap<M, T>(
        &mut self,
        label: T,
        keycode: KeyCode,
        system: impl IntoSystem<(), (), M>,
    ) -> &mut Self
    where
        T: Send + Sync + PartialEq + 'static;
}

impl KeymapperAppExt for App {
    fn add_keymap<M, T>(
        &mut self,
        label: T,
        keycode: KeyCode,
        system: impl IntoSystem<(), (), M>,
    ) -> &mut Self
    where
        T: Send + Sync + PartialEq + 'static,
    {
        if !self.world().contains_resource::<Keymapper<T>>() {
            self.insert_resource(Keymapper::<T>::new(vec![]));
        }

        let mut manager = self.world_mut().resource_mut::<Keymapper<T>>();
        manager.keymaps.push(Keymap::new(label, keycode, system));

        self
    }
}
