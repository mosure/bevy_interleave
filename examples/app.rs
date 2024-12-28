use bevy::prelude::*;

use bevy_interleave::prelude::*;


#[derive(
    Clone,
    Debug,
    Default,
    Planar,
    Reflect,
    ReflectInterleaved,
    StorageBindings,
    // TextureBindings,
)]
pub struct MyStruct {
    // #[texture_format(TextureFormat::R32Sint)]
    pub field: i32,

    // #[texture_format(TextureFormat::R32Uint)]
    pub field2: u32,

    // #[texture_format(TextureFormat::R8Unorm)]
    pub bool_field: bool,

    // #[texture_format(TextureFormat::Rgba32Uint)]
    pub array: [u32; 4],
}


fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(
        PlanarStoragePlugin::<MyStruct>::default(),
    );

    app.run();
}
