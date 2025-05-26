# SendEvents

```rust
use bevy::prelude::*;
use rmv_bevy_testing_tools::prelude::*;
use rstest::rstest;

#[derive(Debug, Default, Event)]
enum MyEvent { #[default] A, B, C }

#[rstest]
fn some_test(#[from(test_app)] mut app: TestApp) {
    app.add_event::<MyEvent>();

    app.send_event_default::<MyEvent>();
    app.send_event(MyEvent::B);
    app.send_event_batch([MyEvent::A, MyEvent::C]);
}
```
