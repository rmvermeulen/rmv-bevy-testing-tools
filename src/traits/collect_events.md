# CollectEvents

```rust
use bevy::prelude::*;
use rmv_bevy_testing_tools::prelude::*;
use rstest::rstest;

#[derive(Debug, Default, Event)]
enum MyEvent { #[default] A, B, C }

#[rstest]
fn some_test(#[from(test_app)] mut app: TestApp) {
    // before test, use 1 of these
    app.collect_events::<MyEvent>();
    app.collect_events_only(MyEvent::This);
    app.collect_events_any_of([MyEvent::A, MyEvent::C]);

    // ...

    // after test
    app.get_collected_events::<MyEvent>();
}
```
