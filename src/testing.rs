pub mod event_collector;

use bevy_app::{App, AppExit, Plugins};
use bevy_asset::AssetApp;
use bevy_asset::AssetPlugin;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::schedule::NextState;
use bevy_ecs::schedule::State;
use bevy_ecs::schedule::States;
use bevy_internal::{utils::default, MinimalPlugins};
use bevy_pbr::MaterialPlugin;
use bevy_pbr::StandardMaterial;
use bevy_render::mesh::MeshPlugin;
use bevy_render::render_resource::Shader;
use bevy_render::texture::ImagePlugin;
use bevy_window::ExitCondition;
use bevy_window::WindowPlugin;
use rstest::{fixture, rstest};

#[derive(Debug, Deref, DerefMut)]
pub struct TestApp(pub App);

impl TestApp {
    pub fn get_state<S: States>(&self) -> Option<&S> {
        self.world.get_resource::<State<S>>().map(|s| s.get())
    }
    pub fn get_next_state<S: States>(&self) -> Option<&Option<S>> {
        self.world.get_resource::<NextState<S>>().map(|s| &s.0)
    }
    pub fn set_next_state<S: States>(&mut self, next: S) -> Option<()> {
        self.world
            .get_resource_mut::<NextState<S>>()
            .map(|mut s| s.set(next))
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        self.world.send_event(AppExit);
    }
}

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
