pub mod prelude;

pub mod interface {
    pub use bevy_interleave_interface::*;
}

pub mod macros {
    pub use bevy_interleave_macros::*;
}



mod tests {
    use std::sync::{Arc, Mutex};

    use bevy::prelude::*;
    use crate::prelude::*;

    #[derive(
        Debug,
        Planar,
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

    #[derive(Resource, Default)]
    struct TestSuccess(Arc<Mutex<bool>>);

    #[allow(dead_code)]
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

        commands.spawn(planar_handle);
    }

    #[allow(dead_code)]
    fn check_bind_group(
        bind_group: Query<&PlanarTextureBindGroup::<PlanarTextureMyStruct>>,
        success: Res<TestSuccess>,
    ) {
        if bind_group.iter().count() > 0 {
            *success.0.lock().unwrap() = true;
        }
    }

    #[allow(dead_code)]
    fn test_timeout(
        mut exit: EventWriter<bevy::app::AppExit>,
        mut frame_count: Local<u32>,
    ) {
        *frame_count += 1;

        if *frame_count > 10 {
            exit.send(bevy::app::AppExit);
        }
    }


    #[test]
    fn texture_bind_group() {
        let mut app = App::new();

        app.add_plugins((
            DefaultPlugins,
            bevy::app::ScheduleRunnerPlugin::run_loop(
                std::time::Duration::from_millis(50)
            ),

            PlanarPlugin::<PlanarMyStruct>::default(),
            PlanarTexturePlugin::<PlanarTextureMyStruct>::default(),
            // TODO: PlanarStoragePlugin::<PlanarStorageMyStruct>::default(),
        ));

        app.add_systems(Startup, setup_planar);

        let render_app = app.sub_app_mut(bevy::render::RenderApp);
        render_app.add_systems(
            bevy::render::Render,
            check_bind_group.in_set(bevy::render::RenderSet::QueueMeshes),
        );

        render_app.init_resource::<TestSuccess>();
        let success = render_app.world.resource::<TestSuccess>();
        let success = success.0.clone();

        app.add_systems(Update, test_timeout);
        app.run();

        if !*success.lock().unwrap() {
            panic!("app exit without success flag set - bind group was not found");
        }
    }
}
