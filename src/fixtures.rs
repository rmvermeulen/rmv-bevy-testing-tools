use bevy::{
    app::{App, Plugins},
    prelude::MinimalPlugins,
    utils::default,
    window::{ExitCondition, WindowPlugin},
};
use rstest::fixture;

use crate::test_app::TestApp;

/// bevy's [`MinimalPlugins`] and a hidden window
#[cfg(any(test, feature = "rstest"))]
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
#[cfg(feature = "rstest")]
#[fixture]
pub fn test_app<P>(
    #[default(())] plugins: impl Plugins<P>,
    #[from(minimal_test_app)] mut app: TestApp,
) -> TestApp {
    use bevy::{
        asset::{AssetApp, AssetPlugin},
        pbr::{MaterialPlugin, StandardMaterial},
        render::{mesh::MeshPlugin, render_resource::Shader, texture::ImagePlugin},
    };
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
    #[cfg(feature = "manage_state")]
    use bevy::state::{
        app::{AppExtStates, StatesPlugin},
        state::{NextState, States},
    };
    use rstest::{fixture, rstest};
    use speculoos::{assert_that, asserting, option::OptionAssertions, string::StrAssertions};

    #[cfg(feature = "rstest")]
    use crate::fixtures::test_app;
    use crate::fixtures::{minimal_test_app, TestApp};
    #[cfg(feature = "manage_state")]
    use crate::traits::ManageState;

    #[rstest]
    fn test_minimal_app_is_created(mut minimal_test_app: TestApp) {
        minimal_test_app.update();
        drop(minimal_test_app);
    }

    #[cfg(feature = "rstest")]
    #[rstest]
    fn test_test_app_is_created(mut test_app: TestApp) {
        test_app.update();
        drop(test_app);
    }

    #[cfg(feature = "manage_state")]
    #[derive(States, Debug, Copy, Clone, PartialEq, Eq, Hash)]
    enum MyState {
        First,
        Second,
    }

    #[cfg(feature = "manage_state")]
    #[fixture]
    fn states_app(
        #[from(minimal_test_app)]
        #[with(StatesPlugin)]
        app: TestApp,
    ) -> TestApp {
        app
    }

    #[cfg(feature = "manage_state")]
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

    #[cfg(feature = "manage_state")]
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

    #[cfg(feature = "manage_state")]
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
