use bevy::prelude::*;

#[derive(Resource)]
pub struct KeymapsManager<T: Send + Sync + 'static> {
    pub keymaps: Vec<Keymap<T>>,
}

impl<T: PartialEq + Send + Sync + 'static> KeymapsManager<T> {
    pub fn remove(&mut self, label: T) {
        self.keymaps.retain(|k| k.label != label);
    }

    pub fn new(keymaps: Vec<Keymap<T>>) -> Self {
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

pub struct Keymap<T> {
    pub label: T,
    pub keycode: KeyCode,
    pub system: Box<dyn System<In = (), Out = ()>>,
    initialized: bool,
}

impl<T> Keymap<T> {
    pub fn new<M>(label: T, keycode: KeyCode, system: impl IntoSystem<(), (), M>) -> Self {
        Self {
            label,
            keycode,
            system: Box::new(IntoSystem::into_system(system)),
            initialized: false,
        }
    }
}

pub fn keymaps_runner_system<T>(world: &mut World)
where
    T: Send + Sync + PartialEq + 'static,
{
    let keyboard_input = world.resource::<ButtonInput<KeyCode>>().clone();
    let keycodes: Vec<KeyCode> = keyboard_input.get_just_pressed().copied().collect();

    let result = world.resource_scope(
        |world,
         mut manager: Mut<KeymapsManager<T>>|
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
        if !self.world().contains_resource::<KeymapsManager<T>>() {
            self.insert_resource(KeymapsManager::<T>::new(vec![]));
        }

        let mut manager = self.world_mut().resource_mut::<KeymapsManager<T>>();
        manager.keymaps.push(Keymap::new(label, keycode, system));

        self
    }
}
