# bevy_interleave 🧩
[![test](https://github.com/mosure/bevy_interleave/workflows/test/badge.svg)](https://github.com/Mosure/bevy_interleave/actions?query=workflow%3Atest)
[![GitHub License](https://img.shields.io/github/license/mosure/bevy_interleave)](https://raw.githubusercontent.com/mosure/bevy_interleave/main/LICENSE)
[![GitHub Releases](https://img.shields.io/github/v/release/mosure/bevy_interleave?include_prereleases&sort=semver)](https://github.com/mosure/bevy_interleave/releases)
[![GitHub Issues](https://img.shields.io/github/issues/mosure/bevy_interleave)](https://github.com/mosure/bevy_interleave/issues)
[![crates.io](https://img.shields.io/crates/v/bevy_interleave.svg)](https://crates.io/crates/bevy_interleave)

bevy support for e2e packed to planar bind groups


## minimal example

```rust
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
```


## why bevy?

`bevy_interleave` simplifies bind group creation within `bevy`. `Planar` derives can be used in conjunction with `ShaderType`'s to support both packed and planar data render pipelines.


## compatible bevy versions

| `bevy_interleave` | `bevy` |
| :--               | :--    |
| `0.1`             | `0.12` |
