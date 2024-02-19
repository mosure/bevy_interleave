use std::marker::PhantomData;

use bevy::{
    prelude::*,
    reflect::GetTypeRegistration,
    render::extract_component::{
        ExtractComponent,
        ExtractComponentPlugin,
    },
};

use crate::{
    PlanarTexture,
    ReflectInterleaved,
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

impl<R> Plugin for PlanarTexturePlugin<R>
where
    R: PlanarTexture + ReflectInterleaved + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect,
    R::PlanarType: Asset,
{
    fn build(&self, app: &mut App) {
        app.register_type::<R>();

        app.add_plugins(ExtractComponentPlugin::<R>::default());

        app.add_systems(Startup, prepare_textures::<R>);

        let render_app = app.sub_app_mut(bevy::render::RenderApp);
        render_app.add_systems(
            bevy::render::Render,
            queue_gpu_texture_buffers::<R>.in_set(bevy::render::RenderSet::PrepareAssets),
        );

        app.init_resource::<PlanarTextureLayouts::<R>>();
    }
}


#[derive(bevy::prelude::Resource, Default)]
pub struct PlanarTextureLayouts<R: PlanarTexture + ReflectInterleaved + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect> {
    pub bind_group_layout: Option<bevy::render::render_resource::BindGroupLayout>,
    pub phantom: PhantomData<fn() -> R>,
}


fn prepare_textures<R>(
    // mut commands: Commands,
    asset_server: Res<AssetServer>,
    cloud_res: Res<Assets<R::PlanarType>>,
    // mut images: ResMut<Assets<Image>>,
    // mut bind_group_layout: ResMut<PlanarTextureLayouts<R>>,
    clouds: Query<
        (
            Entity,
            &Handle<R::PlanarType>,
        ),
        Without<R>,
    >,
)
where
    R: PlanarTexture + ReflectInterleaved + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect,
    R::PlanarType: Asset,
{
    for (_entity, cloud_handle) in clouds.iter() {
        if Some(bevy::asset::LoadState::Loading) == asset_server.get_load_state(cloud_handle){
            continue;
        }

        if cloud_res.get(cloud_handle).is_none() {
            continue;
        }

        let _cloud = cloud_res.get(cloud_handle).unwrap();
    }
}

fn queue_gpu_texture_buffers<R>(
    // mut commands: Commands,
)
where
    R: PlanarTexture + ReflectInterleaved + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect,
    R::PlanarType: Asset,
{
    
}
