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

        app.add_systems(Startup, prepare_textures::<R>);

        let render_app = app.sub_app_mut(bevy::render::RenderApp);
        render_app.add_systems(
            bevy::render::Render,
            queue_gpu_texture_buffers::<R>.in_set(bevy::render::RenderSet::PrepareAssets),
        );

        render_app.init_resource::<PlanarTextureLayouts::<R>>();
        render_app.add_systems(bevy::render::ExtractSchedule, setup_planar_texture_layouts::<R>);
    }
}


#[derive(bevy::prelude::Resource, Default)]
pub struct PlanarTextureLayouts<R: PlanarTexture + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect> {
    pub bind_group_layout: Option<bevy::render::render_resource::BindGroupLayout>,
    pub phantom: PhantomData<fn() -> R>,
}

fn setup_planar_texture_layouts<R>(
    mut layouts: ResMut<PlanarTextureLayouts<R>>,
    render_device: ResMut<bevy::render::renderer::RenderDevice>,
)
where
    R: PlanarTexture + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect,
{
    if layouts.bind_group_layout.is_none() {
        let layout = R::bind_group_layout(
            &render_device,
        );

        layouts.bind_group_layout = Some(layout);
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
            &cloud,
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
    clouds: Query<(
        Entity,
        &R,
    )>,
)
where
    R: PlanarTexture + Default + Component + ExtractComponent + GetTypeRegistration + Clone + Reflect,
    R::PlanarType: Asset,
{
    if bind_group_layout.bind_group_layout.is_none() {
        println!("bind_group_layout is none");
        return;
    }

    for (entity, texture_buffers,) in clouds.iter() {
        let bind_group = texture_buffers.bind_group(
            &render_device,
            &gpu_images,
            bind_group_layout.bind_group_layout.as_ref().unwrap()
        );

        commands.entity(entity).insert(PlanarTextureBindGroup::<R> {
            bind_group,
            phantom: PhantomData,
        });
    }
}
