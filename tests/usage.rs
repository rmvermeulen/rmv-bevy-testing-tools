use rmv_bevy_testing_tools::prelude::*;
use rstest::rstest;

#[rstest]
fn all_things_are_accessible(#[from(default_test_app)] mut app: TestApp) {
    app.update();
}
