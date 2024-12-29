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
    TextureBindings,
)]
pub struct MyStruct {
    #[texture_format(TextureFormat::R32Sint)]
    pub field: i32,

    #[texture_format(TextureFormat::R32Uint)]
    pub field2: u32,

    #[texture_format(TextureFormat::R8Unorm)]
    pub bool_field: bool,

    #[texture_format(TextureFormat::Rgba32Uint)]
    pub array: [u32; 4],
}


fn main() {
    let interleaved = vec![
        MyStruct { field: 0, field2: 1_u32, bool_field: true, array: [0, 1, 2, 3] },
        MyStruct { field: 2, field2: 3_u32, bool_field: false, array: [4, 5, 6, 7] },
        MyStruct { field: 4, field2: 5_u32, bool_field: true, array: [8, 9, 10, 11] },
    ];

    let planar = PlanarMyStruct::from_interleaved(interleaved);

    println!("{:?}", planar.field);
    println!("{:?}", planar.field2);
    println!("{:?}", planar.bool_field);
    println!("{:?}", planar.array);

    // Prints:
    // [0, 2, 4]
    // [1, 3, 5]
    // [true, false, true]
    // [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11]]

    println!("\n\n{:?}", MyStruct::min_binding_sizes());
    println!("{:?}", MyStruct::ordered_field_names());

    // Prints:
    // [4, 4, 1, 16]
    // ["field", "field2", "bool_field", "array"]
}
