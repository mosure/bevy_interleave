use std::marker::PhantomData;

use bevy::{
    prelude::*,
    reflect::GetTypeRegistration,
    render::extract_component::{
        ExtractComponent,
        ExtractComponentPlugin,
    },
};

use crate::PlanarTexture;


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
    R: PlanarTexture + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect,
    R::PlanarType: Asset,
{
    fn build(&self, app: &mut App) {
        app.register_type::<R>();

        app.add_plugins(ExtractComponentPlugin::<R>::default());

        app.add_systems(Update, prepare_textures::<R>);

        let render_app = app.sub_app_mut(bevy::render::RenderApp);
        render_app.add_systems(
            bevy::render::Render,
            queue_gpu_texture_buffers::<R>.in_set(bevy::render::RenderSet::PrepareAssets),
        );
    }

    fn finish(&self, app: &mut App) {
        if let Ok(render_app) = app.get_sub_app_mut(bevy::render::RenderApp) {
            render_app.init_resource::<PlanarTextureLayouts::<R>>();
        }
    }
}


#[derive(bevy::prelude::Resource)]
pub struct PlanarTextureLayouts<R: PlanarTexture + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect> {
    pub bind_group_layout: bevy::render::render_resource::BindGroupLayout,
    pub phantom: PhantomData<fn() -> R>,
}

impl<R: PlanarTexture + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect>
FromWorld for PlanarTextureLayouts<R> {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<bevy::render::renderer::RenderDevice>();

        let bind_group_layout = R::bind_group_layout(
            render_device,
        );

        PlanarTextureLayouts::<R> {
            bind_group_layout,
            phantom: PhantomData,
        }
    }
}

fn prepare_textures<R>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cloud_res: Res<Assets<R::PlanarType>>,
    mut images: ResMut<Assets<Image>>,
    clouds: Query<
        (
            Entity,
            &Handle<R::PlanarType>,
        ),
        Without<R>,
    >,
)
where
    R: PlanarTexture + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect,
    R::PlanarType: Asset,
{
    for (entity, cloud_handle) in clouds.iter() {
        if Some(bevy::asset::LoadState::Loading) == asset_server.get_load_state(cloud_handle){
            continue;
        }

        if cloud_res.get(cloud_handle).is_none() {
            continue;
        }

        let cloud = cloud_res.get(cloud_handle).unwrap();

        let buffers = R::prepare(
            &mut images,
            cloud,
        );

        commands.entity(entity).insert(buffers);
    }
}


#[derive(bevy::prelude::Component, Clone, Debug)]
pub struct PlanarTextureBindGroup<R: PlanarTexture + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect> {
    pub bind_group: bevy::render::render_resource::BindGroup,
    pub phantom: PhantomData<fn() -> R>,
}


fn queue_gpu_texture_buffers<R>(
    mut commands: Commands,
    render_device: ResMut<bevy::render::renderer::RenderDevice>,
    gpu_images: Res<bevy::render::render_asset::RenderAssets<Image>>,
    bind_group_layout: Res<PlanarTextureLayouts<R>>,
    clouds: Query<
        (
            Entity,
            &R,
        ),
        Without<PlanarTextureBindGroup::<R>>,
    >,
)
where
    R: PlanarTexture + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect,
    R::PlanarType: Asset,
{
    let layout = &bind_group_layout.bind_group_layout;

    for (entity, texture_buffers,) in clouds.iter() {
        let not_ready = texture_buffers.get_asset_handles().iter()
            .map(|handle| gpu_images.get(handle).is_none())
            .reduce(|a, b| a || b)
            .unwrap_or(true);

        if not_ready {
            continue;
        }

        let bind_group = texture_buffers.bind_group(
            &render_device,
            &gpu_images,
            layout,
        );

        commands.entity(entity).insert(PlanarTextureBindGroup::<R> {
            bind_group,
            phantom: PhantomData,
        });
    }
}
