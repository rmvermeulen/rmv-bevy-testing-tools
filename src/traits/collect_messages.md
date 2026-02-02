# `CollectMessages`

```rust
use bevy::prelude::*;
use rmv_bevy_testing_tools::prelude::*;
use rstest::rstest;

#[derive(Debug, Default, Message)]
enum MyMessage { #[default] A, B, C }

#[rstest]
fn some_test(#[from(default_test_app)] mut app: TestApp) {
    // before test, use 1 of these
    app.collect_events::<MyMessage>();
    app.collect_events_only(MyMessage::This);
    app.collect_events_any_of([MyMessage::A, MyMessage::C]);

    // ...

    // after test
    app.get_collected_events::<MyMessage>();
}
```
