pub mod planar;
pub mod texture;


pub trait PlanarStorage {
    type PackedType;
    type PlanarType;

    fn bind_group(
        &self,
        render_device: &bevy::render::renderer::RenderDevice,
        layout: &bevy::render::render_resource::BindGroupLayout,
    ) -> bevy::render::render_resource::BindGroup;

    fn bind_group_layout(
        render_device: &bevy::render::renderer::RenderDevice,
        read_only: bool,
    ) -> bevy::render::render_resource::BindGroupLayout;

    fn prepare(
        render_device: &bevy::render::renderer::RenderDevice,
        planar: &Self::PlanarType,
    ) -> Self;
}


pub trait PlanarTexture {
    type PackedType;
    type PlanarType;

    fn bind_group(
        &self,
        render_device: &bevy::render::renderer::RenderDevice,
        gpu_images: &bevy::render::render_asset::RenderAssets<bevy::render::texture::Image>,
        layout: &bevy::render::render_resource::BindGroupLayout,
    ) -> bevy::render::render_resource::BindGroup;

    fn bind_group_layout(
        render_device: &bevy::render::renderer::RenderDevice,
    ) -> bevy::render::render_resource::BindGroupLayout;

    fn prepare(
        images: &mut bevy::asset::Assets<bevy::render::texture::Image>,
        planar: &Self::PlanarType,
    ) -> Self;

    fn get_asset_handles(&self) -> Vec<bevy::asset::Handle<bevy::render::texture::Image>>;
}


pub trait ReflectInterleaved {
    type PackedType;

    fn min_binding_sizes() -> &'static [usize];
    fn ordered_field_names() -> &'static [&'static str];
}


pub trait Planar {
    type PackedType;

    fn get(&self, index: usize) -> Self::PackedType;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn set(&mut self, index: usize, value: Self::PackedType);
    fn to_interleaved(&self) -> Vec<Self::PackedType>;

    fn from_interleaved(packed: Vec<Self::PackedType>) -> Self where Self: Sized;
}
