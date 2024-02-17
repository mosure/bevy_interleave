use bevy_interleave::prelude::*;


#[derive(
    Planar,
    MinBindingSize,
    StorageBindings,
)]
pub struct MyStruct {
    pub field: i32,
    pub field2: u32,
}


fn main() {
    let interleaved = vec![
        MyStruct { field: 0, field2: 1_u32 },
        MyStruct { field: 2, field2: 3_u32 },
        MyStruct { field: 4, field2: 5_u32 },
    ];

    let planar = PlanarMyStruct::from_interleaved(interleaved);

    println!("{:?}", planar.field);
    println!("{:?}", planar.field2);

    // Prints:
    // [0, 2, 4]
    // [1, 3, 5]
}
