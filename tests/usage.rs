use rmv_bevy_testing_tools::prelude::*;
use rstest::rstest;

#[rstest]
fn all_things_are_accessible(#[from(test_app)] mut app: TestApp) {
    app.update();
}
