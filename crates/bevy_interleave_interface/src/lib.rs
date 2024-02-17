
pub trait GpuStoragePlanar {
    type PackedType;
    type PlanarType;

    fn bind_group(
        &self,
        render_device: &bevy::render::renderer::RenderDevice,
        layout: &bevy::render::render_resource::BindGroupLayout,
    ) -> bevy::render::render_resource::BindGroup;

    fn bind_group_layout(
        &self,
        render_device: &bevy::render::renderer::RenderDevice,
        read_only: bool,
    ) -> bevy::render::render_resource::BindGroupLayout;

    fn ordered_field_names(&self) -> &'static [&'static str];

    fn prepare(
        render_device: &bevy::render::renderer::RenderDevice,
        planar: &Self::PlanarType,
    ) -> Self;
}


pub trait MinBindingSize {
    type PackedType;

    fn min_binding_sizes() -> &'static [usize];
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
