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

    pub fn run(&mut self, world: &mut World, keycode: KeyCode) -> Result<(), Box<bevy::ecs::system::RunSystemError>> {
        for keymap in &mut self.keymaps {
            if keymap.keycode == keycode {
                keymap.system.run((), world)?;
                keymap.system.apply_deferred(world);
            }
        }

        Ok(())
    }
}

pub struct Keymap {
    pub keycode: KeyCode,
    pub system: Box<dyn System<In = (), Out = ()>>,
}

pub trait Environment: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub fn keymaps_runner_system(
    world: &mut World
) -> Result<(), Box<bevy::ecs::system::RunSystemError>> {
    let keyboard_input = world.resource::<ButtonInput<KeyCode>>().clone();
    let keycodes: Vec<KeyCode> = keyboard_input.get_just_pressed().copied().collect();

    world.resource_scope(|world, mut manager: Mut<KeymapsManager>| -> Result<(), Box<bevy::ecs::system::RunSystemError>> {
        for keycode in keycodes {
            manager.run(world, keycode)?;
        }

        Ok(())
    })?;

    Ok(())
}
