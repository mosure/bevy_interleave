pub mod storage;
pub mod texture;


// TODO: this needs to be refactored and better structured
pub trait PlanarHandle<T>
where
    Self: bevy::ecs::component::Component,
    Self: Clone,
    Self: Default,
    Self: bevy::reflect::FromReflect,
    Self: bevy::reflect::GetTypeRegistration,
    Self: bevy::reflect::Reflect,
    T: bevy::asset::Asset,
{
    fn handle(&self) -> &bevy::asset::Handle<T>;
}

// #[cfg(feature = "debug_gpu")]
// pub debug_gpu: PlanarType,
// TODO: when `debug_gpu` feature is enabled, add a function to access the main -> render world copied asset (for ease of test writing)
pub trait GpuPlanarStorage {
    type PackedType;

    fn len(&self) -> usize;
    fn draw_indirect_buffer(&self) -> &bevy::render::render_resource::Buffer;

    fn bind_group(
        &self,
        render_device: &bevy::render::renderer::RenderDevice,
        layout: &bevy::render::render_resource::BindGroupLayout,
    ) -> bevy::render::render_resource::BindGroup;

    fn bind_group_layout(
        render_device: &bevy::render::renderer::RenderDevice,
        read_only: bool,
    ) -> bevy::render::render_resource::BindGroupLayout;
}

pub trait PlanarStorage
where
    Self: Default,
    Self: Send,
    Self: Sync,
    Self: bevy::reflect::Reflect,
    Self: 'static,
{
    type PackedType;  // Self
    type PlanarType: bevy::asset::Asset + bevy::reflect::GetTypeRegistration + bevy::reflect::FromReflect;
    type PlanarTypeHandle: PlanarHandle<Self::PlanarType>;
    type GpuPlanarType: GpuPlanarStorage + bevy::render::render_asset::RenderAsset<SourceAsset = Self::PlanarType>;
}


// TODO: refactor planar texture to be more like planar storage
pub trait PlanarTexture {
    type PackedType;  // Self
    type PlanarType: bevy::asset::Asset;
    type PlanarTypeHandle: PlanarHandle<Self::PlanarType>;

    // note: planar texture's gpu type utilizes bevy's image render asset

    fn bind_group(
        &self,
        render_device: &bevy::render::renderer::RenderDevice,
        gpu_images: &bevy::render::render_asset::RenderAssets<bevy::render::texture::GpuImage>,
        layout: &bevy::render::render_resource::BindGroupLayout,
    ) -> bevy::render::render_resource::BindGroup;

    fn bind_group_layout(
        render_device: &bevy::render::renderer::RenderDevice,
    ) -> bevy::render::render_resource::BindGroupLayout;

    fn prepare(
        images: &mut bevy::asset::Assets<bevy::image::Image>,
        planar: &Self::PlanarType,
    ) -> Self;

    fn get_asset_handles(&self) -> Vec<bevy::asset::Handle<bevy::image::Image>>;
}


pub trait ReflectInterleaved {
    type PackedType;

    fn min_binding_sizes() -> &'static [usize];
    fn ordered_field_names() -> &'static [&'static str];
}


pub trait Planar {
    type PackedType;

    fn get(&self, index: usize) -> Self::PackedType;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn len(&self) -> usize;
    fn set(&mut self, index: usize, value: Self::PackedType);
    fn to_interleaved(&self) -> Vec<Self::PackedType>;

    fn from_interleaved(packed: Vec<Self::PackedType>) -> Self where Self: Sized;

    fn subset(&self, indices: &[usize]) -> Self;
}
