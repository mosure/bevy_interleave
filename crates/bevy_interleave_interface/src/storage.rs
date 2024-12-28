use std::marker::PhantomData;

use bevy::{
    prelude::*,
    reflect::GetTypeRegistration,
    render::extract_component::ExtractComponentPlugin,
};

use crate::{
    GpuPlanarStorage,
    PlanarHandle,
    PlanarStorage,
};


pub struct PlanarStoragePlugin<R> {
    phantom: PhantomData<fn() -> R>,
}
impl<R> Default for PlanarStoragePlugin<R> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<R: 'static> Plugin for PlanarStoragePlugin<R>
where
    R: PlanarStorage + Default + GetTypeRegistration + Clone + Reflect,
{
    fn build(&self, app: &mut App) {
        app.register_type::<R>();

        app.register_type::<R::PlanarType>();
        app.init_asset::<R::PlanarType>();
        app.register_asset_reflect::<R::PlanarType>();

        app.add_plugins(bevy::render::render_asset::RenderAssetPlugin::<R::GpuPlanarType>::default());
        app.add_plugins(ExtractComponentPlugin::<R::PlanarTypeHandle>::default());

        let render_app = app.sub_app_mut(bevy::render::RenderApp);
        render_app.add_systems(
            bevy::render::Render,
            queue_gpu_storage_buffers::<R>.in_set(bevy::render::RenderSet::PrepareBindGroups),
        );
    }

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(bevy::render::RenderApp) {
            render_app.init_resource::<PlanarStorageLayouts::<R>>();
        }
    }
}


#[derive(bevy::prelude::Resource)]
pub struct PlanarStorageLayouts<R: PlanarStorage> {
    pub bind_group_layout: bevy::render::render_resource::BindGroupLayout,
    pub phantom: PhantomData<fn() -> R>,
}

impl<R: PlanarStorage>
FromWorld for PlanarStorageLayouts<R> {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<bevy::render::renderer::RenderDevice>();

        let read_only = true;
        let bind_group_layout = R::GpuPlanarType::bind_group_layout(
            render_device,
            read_only,
        );

        Self {
            bind_group_layout,
            phantom: PhantomData,
        }
    }
}

#[derive(bevy::prelude::Component, Clone, Debug)]
pub struct PlanarStorageBindGroup<R: PlanarStorage> {
    pub bind_group: bevy::render::render_resource::BindGroup,
    pub phantom: PhantomData<fn() -> R>,
}


fn queue_gpu_storage_buffers<R>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    render_device: ResMut<bevy::render::renderer::RenderDevice>,
    gpu_planars: Res<bevy::render::render_asset::RenderAssets<R::GpuPlanarType>>,
    bind_group_layout: Res<PlanarStorageLayouts<R>>,
    clouds: Query<
        (
            Entity,
            &R::PlanarTypeHandle,
        ),
        Without<PlanarStorageBindGroup::<R>>,
    >,
)
where
    R: PlanarStorage + Default + Clone + Reflect,
    R::PlanarType: Asset,
{
    let layout = &bind_group_layout.bind_group_layout;

    info!("queue_gpu_storage_buffers");

    for (entity, planar_handle,) in clouds.iter() {
        info!("handle {:?}", planar_handle.handle());

        if let Some(load_state) = asset_server.get_load_state(planar_handle.handle()) {
            if load_state.is_loading() {
                info!("loading");
                continue;
            }
        }

        if gpu_planars.get(planar_handle.handle()).is_none() {
            info!("no gpu planar");
            continue;
        }

        let gpu_planar: &<R as PlanarStorage>::GpuPlanarType = gpu_planars.get(planar_handle.handle()).unwrap();
        let bind_group = gpu_planar.bind_group(
            &render_device,
            layout,
        );

        commands.entity(entity).insert(PlanarStorageBindGroup::<R> {
            bind_group,
            phantom: PhantomData,
        });
    }
}
