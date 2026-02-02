# rmv-bevy-testing-tools

## `TestApp`

`TestApp` wraps a `bevy::app::App` so it can exit cleanly when dropped, and
implements some helpful traits.

This crate comes with two pre-defined test-apps (see [`./src/fixtures.rs`]):

- `minimal_test_app` which disables the window and exit-condition
- `test_app` which hopefully captures enough of bevy's default setup
  to run most tests cases with minimal clutter.

The traits:

- [basic_query](/src/traits/basic_query.md)
- [collect_messages](/src/traits/collect_messages.md)
- [immediate_query](/src/traits/immediate_query.md)
- [manage_state](/src/traits/manage_state.md)
- [write_messages](/src/traits/write_messages.md)

### Basic example

```rust
use bevy::prelude::*;
use rmv_bevy_testing_tools::prelude::*;
use rstest::{fixture, rstest};
use speculoos::prelude::*;

#[derive(Message, Default)]
struct MyMessage;

pub fn my_game_plugin(app: &mut App) {
    app.add_message::<MyMessage>()
        .add_systems(Update, (|mut writer: MessageWriter<MyMessage>| {
            writer.write_default();
        }).run_if(|mut count: Local<usize>| {
            let pass = (*count % 8) == 0;
            *count += 1;
            pass
        }));
}

#[rstest]
fn test_my_system(#[from(default_test_app)] #[with(my_game_plugin)] app: TestApp) {
    for _ in 0..1000 {
        app.update();
    }
    assert_that!(app.get_collected_messages::<MyMessage>()).is_ok().has_length(125);
}
```

### Custom setup

```rust
use bevy_app::{App, Plugins};
use rstest::{fixture, rstest};
use rmv_bevy_testing_tools::prelude::*;

// setup a reusable fixture with some combination of plugins
#[fixture]
pub fn my_custom_test_app() -> TestApp {
    let mut app = App::new();
    app.add_plugins((
        // add your plugins
    ));
    TestApp(app)
}

// this fixture can take more plugins as argument.
// `test_app` and `minimal_test_app` also use this pattern
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

fn my_simple_plugin(app: &mut App) {}
fn my_game_plugin(app: &mut App) {}

#[rstest]
fn test_my_simple_plugin(#[with(MySimplePlugin)] mut test_app: TestApp) {
    // run system tests
}
#[rstest]
fn test_my_game_plugin(#[with(MyGamePlugin)] mut test_app: TestApp) {
    // run systems tests involving assets
}

```
