pub mod prelude;

pub mod interface {
    pub use bevy_interleave_interface::*;
}

pub mod macros {
    pub use bevy_interleave_macros::*;
}



#[allow(dead_code)]
mod tests {
    use std::sync::{Arc, Mutex};

    use bevy::prelude::*;
    use crate::prelude::*;

    #[derive(
        Clone,
        Debug,
        Default,
        Reflect,
        Planar,
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

    #[derive(Resource, Default)]
    struct TestSuccess(Arc<Mutex<bool>>);

    fn setup_planar(
        mut commands: Commands,
        mut gaussian_assets: ResMut<Assets<PlanarMyStruct>>,
    ) {
        let planar = PlanarMyStruct::from_interleaved(vec![
            MyStruct { field: 0, field2: 1_u32, bool_field: true, array: [0, 1, 2, 3] },
            MyStruct { field: 2, field2: 3_u32, bool_field: false, array: [4, 5, 6, 7] },
            MyStruct { field: 4, field2: 5_u32, bool_field: true, array: [8, 9, 10, 11] },
        ]);

        let planar_handle = gaussian_assets.add(planar);

        commands.spawn(PlanarMyStructHandle(planar_handle));
    }

    // TODO: require both texture and storage bind groups
    // fn check_texture_bind_group(
    //     bind_group: Query<&PlanarTextureBindGroup::<PlanarTextureMyStruct>>,
    //     success: Res<TestSuccess>,
    // ) {
    //     if bind_group.iter().count() > 0 {
    //         *success.0.lock().unwrap() = true;
    //     }
    // }

    fn check_storage_bind_group(
        bind_group: Query<&PlanarStorageBindGroup::<MyStruct>>,
        success: Res<TestSuccess>,
    ) {
        if bind_group.iter().count() > 0 {
            *success.0.lock().unwrap() = true;
        }
    }

    fn test_timeout(
        mut exit: EventWriter<bevy::app::AppExit>,
        mut frame_count: Local<u32>,
    ) {
        *frame_count += 1;

        if *frame_count > 5 {
            exit.send(bevy::app::AppExit::Success);
        }
    }


    #[test]
    fn texture_bind_group() {
        let mut app = App::new();

        app.add_plugins(
            DefaultPlugins
                .set(bevy::app::ScheduleRunnerPlugin::run_loop(
                    std::time::Duration::from_millis(50)
                )),
        );
        app.add_plugins(
            PlanarStoragePlugin::<MyStruct>::default(),
            // PlanarTexturePlugin::<PlanarTextureMyStruct>::default(),
        );

        app.add_systems(Startup, setup_planar);

        let render_app = app.sub_app_mut(bevy::render::RenderApp);
        render_app.add_systems(
            bevy::render::Render,
            (
                check_storage_bind_group.in_set(bevy::render::RenderSet::QueueMeshes),
                // check_texture_bind_group.in_set(bevy::render::RenderSet::QueueMeshes)
            ),
        );

        let success = TestSuccess(Arc::new(Mutex::new(false)));
        let success_arc = success.0.clone();
        render_app.insert_resource(success);

        app.add_systems(Update, test_timeout);
        app.run();

        if !*success_arc.lock().unwrap() {
            panic!("app exit without success flag set - bind group was not found");
        }
    }
}
