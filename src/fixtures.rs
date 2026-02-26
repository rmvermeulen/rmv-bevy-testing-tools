use bevy_app::{App, Plugins};
use bevy_utils::default;
use bevy_window::{ExitCondition, WindowPlugin};
use rstest::fixture;

use crate::test_app::TestApp;

/// bevy's [`MinimalPlugins`] and a hidden window
#[cfg(any(test, feature = "minimal"))]
#[fixture]
pub fn minimal_test_app<P>(#[default(())] additional_plugins: impl Plugins<P>) -> TestApp {
    use bevy_internal::MinimalPlugins;

    let mut app = App::new();

    app.add_plugins((MinimalPlugins, additional_plugins));
    if !app.is_plugin_added::<WindowPlugin>() {
        app.add_plugins(WindowPlugin {
            primary_window: None,
            exit_condition: ExitCondition::DontExit,
            ..default()
        });
    }
    TestApp(app)
}

/// minimal_test_app + basic assets
#[cfg(feature = "rstest")]
#[fixture]
pub fn default_test_app<P>(
    #[default(())] additional_plugins: impl Plugins<P>,
    #[from(minimal_test_app)] mut app: TestApp,
) -> TestApp {
    if std::env::var("DISPLAY").is_ok() {
        use bevy_a11y::AccessibilityPlugin;
        use bevy_asset::AssetPlugin;
        use bevy_image::ImagePlugin;
        use bevy_input::InputPlugin;
        use bevy_winit::WinitPlugin;
        app.add_plugins((
            // to load cursor images
            (AssetPlugin::default(), ImagePlugin::default()),
            // to deal with keyboard focus
            InputPlugin,
            // required for window/monitor stuff
            AccessibilityPlugin,
            // to run the app
            WinitPlugin {
                run_on_any_thread: true,
            },
        ))
        .add_plugins(additional_plugins);
    } else {
        eprintln!("no DISPLAY, skipping...");
    }
    app
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::time::common_conditions::once_after_delay;
    use bevy_app::{App, AppExit, Plugin, Update};
    use bevy_ecs::{message::MessageWriter, schedule::IntoScheduleConfigs};
    use rstest::rstest;
    use speculoos::assert_that;

    #[cfg(feature = "rstest")]
    use crate::fixtures::default_test_app;
    use crate::fixtures::{TestApp, minimal_test_app};

    fn app_timeout_plugin(duration: Duration) -> impl Plugin {
        fn write_app_exit(mut app_exit: MessageWriter<AppExit>) {
            app_exit.write_default();
        }
        move |app: &mut App| {
            app.add_systems(Update, write_app_exit.run_if(once_after_delay(duration)));
        }
    }

    #[rstest]
    #[async_std::test]
    #[timeout(Duration::from_millis(40))]
    async fn test_minimal_app_can_run(
        #[from(minimal_test_app)]
        #[with(app_timeout_plugin(Duration::from_millis(20)))]
        mut app: TestApp,
    ) {
        let exit = app.run();
        assert_that!(exit)
            .named("AppExit within timeout")
            .is_equal_to(AppExit::Success);
    }

    #[cfg(feature = "rstest")]
    #[rstest]
    #[async_std::test]
    #[timeout(Duration::from_millis(40))]
    async fn test_default_test_app_can_run(#[from(default_test_app)] mut app: TestApp) {
        let exit = app
            .add_plugins(app_timeout_plugin(Duration::from_millis(30)))
            .run();
        assert_that!(exit)
            .named("AppExit within timeout")
            .is_equal_to(AppExit::Success);
    }
}
