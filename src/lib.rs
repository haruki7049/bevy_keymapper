pub use bevy_keymapper_derive::Environment;

use bevy::prelude::*;
use std::any::Any;

#[derive(Resource)]
pub struct KeymapsManager {
    pub keymaps: Vec<Keymap>,
}

impl KeymapsManager {
    pub fn new(keymaps: Vec<Keymap>) -> Self {
        Self { keymaps }
    }
}

pub struct Keymap {
    pub keycode: KeyCode,
    pub function: Box<dyn Fn(&mut Commands, &dyn Environment) + Send + Sync>,
}

pub trait Environment: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

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
