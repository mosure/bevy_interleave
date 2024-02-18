use std::marker::PhantomData;

use bevy::{
    prelude::*,
    reflect::GetTypeRegistration,
    render::extract_component::{
        ExtractComponent,
        ExtractComponentPlugin,
    },
};


pub struct PlanarTexturePlugin<R> {
    phantom: PhantomData<fn() -> R>,
}
impl<R> Default for PlanarTexturePlugin<R> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<R: Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect> Plugin for PlanarTexturePlugin<R> {
    fn build(&self, app: &mut App) {
        app.register_type::<R>();

        app.add_plugins(ExtractComponentPlugin::<R>::default());

        // TODO: add queuing system registration /w binding? (seems to require too much custom logic for derive e.g. SH texture planes)
        //       ^ can use MinBindingSize to determine the number of planes
        //       will require the system to be generated in the macro (bound here)
    }
}
