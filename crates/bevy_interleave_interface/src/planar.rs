use std::marker::PhantomData;

use bevy::{
    prelude::*,
    reflect::GetTypeRegistration,
};

use crate::Planar;


pub struct PlanarPlugin<R> {
    phantom: PhantomData<fn() -> R>,
}
impl<R> Default for PlanarPlugin<R> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<R> Plugin for PlanarPlugin<R>
where
    R: Planar + Default + Asset + GetTypeRegistration + Clone + Reflect + FromReflect,
{
    fn build(&self, app: &mut App) {
        app.register_type::<R>();
        app.init_asset::<R>();
        app.register_asset_reflect::<R>();
    }
}
