use std::time::Duration;

use rmv_bevy_testing_tools::prelude::*;
use rstest::rstest;

#[rstest]
#[async_std::test]
#[timeout(Duration::from_millis(10))]
async fn all_things_are_accessible(#[from(default_test_app)] mut app: TestApp) {
    app.update();
}
