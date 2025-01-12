# rmv-bevy-testing-tools

## TestApp

```rs
use super::{MySimplePlugin, MyGamePlugin};
use rstest::rstest;
use rmv_bevy_testing_tools::{test_app, TestApp};

#[rstest]
fn test_my_simple_plugin(#[with(MySimplePlugin)] mut minimal_test_app: TestApp) {
    // run system tests
}
#[rstest]
fn test_my_plugin(#[with(MyGamePlugin)] mut test_app: TestApp) {
    // run systems tests involving assets
}

#[fixture]
pub fn my_custom_test_app() -> TestApp {
    let mut app = App::new();
    app.add_plugins((
        // add your plugins
    ));
    TestApp(app)
}

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

```

## EventCollector

```rs

use rmv_bevy_testing_tools::{test_app, EventCollector, GetCollectedEvents, TestApp};
use rstest::rstest;
use speculoos::{assert_that, option::OptionAssertions};

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
