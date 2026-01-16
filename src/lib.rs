use bevy::prelude::*;

#[derive(Resource)]
pub struct KeymapsManager {
    pub keymaps: Vec<Keymap>,
}

impl KeymapsManager {
    pub fn new(keymaps: Vec<Keymap>) -> Self {
        Self { keymaps }
    }

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

pub struct Keymap {
    pub keycode: KeyCode,
    pub system: Box<dyn System<In = (), Out = ()>>,
    initialized: bool,
}

impl Keymap {
    pub fn new<M>(keycode: KeyCode, system: impl IntoSystem<(), (), M>) -> Self {
        Self {
            keycode,
            system: Box::new(IntoSystem::into_system(system)),
            initialized: false,
        }
    }
}

pub fn keymaps_runner_system(world: &mut World) {
    let keyboard_input = world.resource::<ButtonInput<KeyCode>>().clone();
    let keycodes: Vec<KeyCode> = keyboard_input.get_just_pressed().copied().collect();

    let result = world.resource_scope(
        |world,
         mut manager: Mut<KeymapsManager>|
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

pub trait KeymapperAppExt {
    fn add_keymap<M>(&mut self, keycode: KeyCode, system: impl IntoSystem<(), (), M>) -> &mut Self;
}

impl KeymapperAppExt for App {
    fn add_keymap<M>(&mut self, keycode: KeyCode, system: impl IntoSystem<(), (), M>) -> &mut Self {
        if !self.world().contains_resource::<KeymapsManager>() {
            self.insert_resource(KeymapsManager::new(vec![]));
            self.add_systems(Update, keymaps_runner_system);
        }

        let system = IntoSystem::into_system(system);
        let mut manager = self.world_mut().resource_mut::<KeymapsManager>();
        manager.keymaps.push(Keymap::new(keycode, system));

        self
    }
}
