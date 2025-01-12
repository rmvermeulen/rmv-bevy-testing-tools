use bevy_app::{App, AppExit, Plugins};
use bevy_asset::{AssetApp, AssetPlugin};
use bevy_derive::{Deref, DerefMut};
use bevy_internal::{utils::default, MinimalPlugins};
use bevy_pbr::{MaterialPlugin, StandardMaterial};
use bevy_render::{mesh::MeshPlugin, render_resource::Shader, texture::ImagePlugin};
use bevy_state::state::{FreelyMutableState, NextState, State, States};
use bevy_window::{ExitCondition, WindowPlugin};
use rstest::{fixture, rstest};

#[derive(Debug, Deref, DerefMut)]
pub struct TestApp(pub App);

impl TestApp {
    pub fn get_state<S: States>(&self) -> Option<&S> {
        self.world().get_resource::<State<S>>().map(|s| s.get())
    }
    pub fn get_next_state<S: FreelyMutableState>(&self) -> Option<&NextState<S>> {
        self.world().get_resource::<NextState<S>>()
    }
    pub fn set_next_state<S: FreelyMutableState>(&mut self, next: S) -> Option<()> {
        self.world_mut()
            .get_resource_mut::<NextState<S>>()
            .map(|mut s| s.set(next))
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        self.world_mut().send_event(AppExit::Success);
    }
}

/// bevy's MinimalPlugins and a hidden window
#[fixture]
pub fn minimal_test_app<P>(#[default(())] plugins: impl Plugins<P>) -> TestApp {
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins,
        WindowPlugin {
            primary_window: None,
            exit_condition: ExitCondition::DontExit,
            ..default()
        },
        plugins,
    ));

    TestApp(app)
}

/// minimal_test_app + basic assets
#[fixture]
pub fn test_app<P>(
    #[default(())] plugins: impl Plugins<P>,
    #[from(minimal_test_app)] mut app: TestApp,
) -> TestApp {
    app.add_plugins(AssetPlugin::default())
        .init_asset::<Shader>()
        .add_plugins((
            MeshPlugin,
            MaterialPlugin::<StandardMaterial>::default(),
            ImagePlugin::default(),
        ))
        .add_plugins(plugins);
    app
}

#[rstest]
fn test_minimal_app_is_created(mut minimal_test_app: TestApp) {
    minimal_test_app.update();
    drop(minimal_test_app);
}

#[rstest]
fn test_test_app_is_created(mut test_app: TestApp) {
    test_app.update();
    drop(test_app);
}
