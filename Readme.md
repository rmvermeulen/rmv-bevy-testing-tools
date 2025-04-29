# rmv-bevy-testing-tools

## TestApp

`TestApp` wraps a `bevy::app::App` so it can exit cleanly when dropped, and
add implements some helpful traits.

```rust
# use bevy_app::{App, Plugin, Plugins};
use rstest::{fixture, rstest};
use rmv_bevy_testing_tools::prelude::*;

# struct MySimplePlugin;
# impl Plugin for MySimplePlugin { fn build(&self, _: &mut App) { } }
# struct MyGamePlugin ;
# impl Plugin for MyGamePlugin { fn build(&self, _: &mut App) { } }

#[rstest]
fn test_my_simple_plugin(#[with(MySimplePlugin)] mut test_app: TestApp) {
    // run system tests
}
#[rstest]
fn test_my_game_plugin(#[with(MyGamePlugin)] mut test_app: TestApp) {
    // run systems tests involving assets
}

// setup a reusable fixture with some combination of plugins
#[fixture]
pub fn my_custom_test_app() -> TestApp {
    let mut app = App::new();
    app.add_plugins((
        // add your plugins
    ));
    TestApp(app)
}

// this fixture can take more plugins as argument
#[fixture]
pub fn my_configurable_custom_test_app<P>(
    #[default(())]
    plugins: impl Plugins<P>
) -> TestApp {
    let mut app = App::new();

    app.add_plugins((
        // add your plugins
        plugins,
    ));

    TestApp(app)
}

#[rstest]
fn test_my_configurable_custom_test_app(
    #[from(my_configurable_custom_test_app)]
    #[with((some::SomePlugin, another::SomePlugin))]
    app: TestApp) {
    // ...
}

```

## EventCollector

```rust
# use bevy_app::{App, Plugins};
# use bevy_ecs::event::Event;
# use rstest::*;
use speculoos::prelude::*;
use rmv_bevy_testing_tools::prelude::*;

#[derive(Event, Clone, Debug, PartialEq)]
struct MyEvent;

#[rstest]
fn test_events(
    #[with((
        GamePlugin,
        EventCollector::<MyEvent>::new()
    ))]
    mut test_app: TestApp,
) {
    test_app.update();
    let events = test_app.get_collected_events::<MyEvent>();
    assert_that!(&events)
        .named("collected events")
        .is_some()
        .is_equal_to(vec![]);
}
```
