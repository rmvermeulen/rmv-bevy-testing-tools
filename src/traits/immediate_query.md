# `ImmediateQuery`

```rust
use bevy::prelude::*;
use rmv_bevy_testing_tools::prelude::*;
use rstest::rstest;

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
struct MyComponent(i32);

#[rstest]
fn some_test(#[from(test_app)] mut app: TestApp) {
    assert_eq!(
        app.query_single::<&MyComponent>().ok().copied(),
        Some(MyComponent(23)));
    assert_eq!(
        app.query_single_filtered::<&MyComponent, With<Transform>>().ok(),
        Some(&MyComponent(23)));
    #[cfg(feature = "iter_tools")]
    assert_eq!(app.query_vec::<&MyComponent>(), vec![&MyComponent(23)]);
}
```
