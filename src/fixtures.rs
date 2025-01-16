use bevy_app::{App, AppExit, Plugins};
use bevy_asset::{AssetApp, AssetPlugin};
use bevy_derive::{Deref, DerefMut};
use bevy_internal::{utils::default, MinimalPlugins};
use bevy_pbr::{MaterialPlugin, StandardMaterial};
use bevy_render::{mesh::MeshPlugin, render_resource::Shader, texture::ImagePlugin};
use bevy_state::state::{FreelyMutableState, NextState, State, States};
use bevy_window::{ExitCondition, WindowPlugin};
use rstest::fixture;

mod traits;
pub use traits::*;

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

#[cfg(test)]
mod tests {
    use bevy_state::{
        app::{AppExtStates, StatesPlugin},
        state::{NextState, States},
    };
    use rstest::{fixture, rstest};
    use speculoos::{assert_that, asserting, option::OptionAssertions, string::StrAssertions};

    use crate::{minimal_test_app, test_app, TestApp};

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

    #[derive(States, Debug, Copy, Clone, PartialEq, Eq, Hash)]
    enum MyState {
        First,
        Second,
    }

    #[fixture]
    fn states_app(
        #[from(minimal_test_app)]
        #[with(StatesPlugin)]
        app: TestApp,
    ) -> TestApp {
        app
    }

    #[rstest]
    fn test_app_get_state(#[from(states_app)] mut app: TestApp) {
        asserting!("TestApp::get_state() before MyState exists")
            .that(&app.get_state::<MyState>())
            .is_none();

        app.insert_state(MyState::First);

        asserting!("TestApp::get_state() when MyState exists")
            .that(&app.get_state::<MyState>())
            .is_some()
            .is_equal_to(&MyState::First);
    }

    #[rstest]
    fn test_app_get_next_state(#[from(states_app)] mut app: TestApp) {
        asserting!("TestApp::get_next_state() before MyState exists")
            .that(&app.get_next_state::<MyState>())
            .is_none();

        app.insert_state(MyState::First);

        let next_state = app.get_next_state::<MyState>();

        asserting!("TestApp::get_next_state() when MyState exists")
            .that(&next_state)
            .is_some();
        assert_that!(format!("{:?}", next_state.unwrap()))
            .is_equal_to(format!("{:?}", NextState::<MyState>::Unchanged));
    }

    #[rstest]
    fn test_app_set_next_state(#[from(states_app)] mut app: TestApp) {
        asserting!("TestApp::set_next_state() before MyState exists")
            .that(&app.set_next_state(MyState::First))
            .is_none();

        app.insert_state(MyState::First);
        asserting!("TestApp::set_next_state() before MyState exists")
            .that(&app.set_next_state(MyState::Second))
            .is_some();

        let next_state = app.get_next_state::<MyState>();
        asserting!("TestApp::get_next_state() after set_next_state()")
            .that(&next_state)
            .is_some();
        assert_that!(format!("{:?}", next_state.unwrap()))
            .contains(format!("{:?}", MyState::Second));
    }
}
