# BasicQuery

```rust
use bevy::prelude::*;
use rmv_bevy_testing_tools::prelude::*;
use rstest::rstest;
#[rstest]
fn some_test(#[from(test_app)] mut app: TestApp) {
    // ...
    if app.query_any::<&Camera, _>() {
        // an entity with Camera exists
    }
    if app.query_any::<(&Camera, &Transform), _>() {
        // an entity with Camera and Transform exists
    }
}
```
