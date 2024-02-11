use bevy_interleave::prelude::*;


#[derive(Planar)]
pub struct MyStruct {
    pub field: i32,
    pub field2: i32,
}


fn main() {
    let interleaved = vec![
        MyStruct { field: 0, field2: 1 },
        MyStruct { field: 2, field2: 3 },
        MyStruct { field: 4, field2: 5 },
    ];

    let planar = PlanarMyStruct::from_interleaved(interleaved);

    println!("{:?}", planar.field);
    println!("{:?}", planar.field2);

    // Prints:
    // [0, 2, 4]
    // [1, 3, 5]
}
