use bevy_app::{App, Plugins};
use bevy_utils::default;
use bevy_window::{ExitCondition, WindowPlugin};
use rstest::fixture;

use crate::test_app::TestApp;

/// bevy's [`MinimalPlugins`] and a hidden window
#[cfg(any(test, feature = "rstest"))]
#[fixture]
pub fn minimal_test_app<P>(#[default(())] plugins: impl Plugins<P>) -> TestApp {
    use bevy_internal::MinimalPlugins;

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
    use bevy_asset::{AssetApp, AssetPlugin};
    use bevy_image::ImagePlugin;
    use bevy_mesh::MeshPlugin;
    use bevy_pbr::{MaterialPlugin, StandardMaterial};
    use bevy_shader::Shader;
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
    use rstest::rstest;

    #[cfg(feature = "rstest")]
    use crate::fixtures::test_app;
    use crate::fixtures::{TestApp, minimal_test_app};

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
}
