# `WriteMessages`

```rust
use bevy::prelude::*;
use rmv_bevy_testing_tools::prelude::*;
use rstest::rstest;

#[derive(Debug, Default, Message)]
enum MyMessage { #[default] A, B, C }

#[rstest]
fn some_test(#[from(test_app)] mut app: TestApp) {
    app.add_event::<MyMessage>();

    app.write_message_default::<MyMessage>();
    app.write_message(MyMessage::B);
    app.write_message_batch([MyMessage::A, MyMessage::C]);
}
```
