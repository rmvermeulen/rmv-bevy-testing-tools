# `ManageState`

```rust
use bevy::prelude::*;
use rmv_bevy_testing_tools::prelude::*;
use rstest::rstest;
use insta::assert_compact_debug_snapshot;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
enum MyState { #[default] A, B, C }

// TODO: clean up these shenanigans
#[rstest] fn some_test(#[from(test_app)] mut app: TestApp) {
# }
# fn run_assertions(mut app: TestApp) {
    app.init_state::<MyState>();

    assert_eq!(app.get_state::<MyState>(), Some(&MyState::A));
    assert_compact_debug_snapshot!(
        app.get_next_state::<MyState>(),
        @"Some(Unchanged)");

    // set next state
    app.set_next_state(MyState::C);
    assert_compact_debug_snapshot!(
        app.get_next_state::<MyState>(),
        @"Some(Pending(C))");
}
# run_assertions(test_app((bevy_state::app::StatesPlugin), minimal_test_app(())));
```
