#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

// TODO: setup tags for different versions of bevy

#[cfg(feature = "speculoos")]
pub mod assertions;
#[cfg(feature = "events")]
pub mod events;
#[cfg(any(test, feature = "rstest"))]
pub mod fixtures;
#[allow(unused_imports)] // silence warning about name starting with test_
pub mod test_app;
pub mod traits;

#[cfg(feature = "insta")]
#[macro_export]
macro_rules! set_snapshot_suffix {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(format!($($expr,)*));
        let _guard = settings.bind_to_scope();
    }
}

pub mod prelude {
    #[cfg(feature = "speculoos")]
    pub use super::assertions::*;
    #[cfg(feature = "insta")]
    pub use super::events::*;
    #[cfg(any(test, feature = "rstest"))]
    pub use super::fixtures::*;
    #[cfg(feature = "insta")]
    pub use super::set_snapshot_suffix;
    pub use super::test_app::*;
    #[allow(unused_imports)]
    pub use super::traits::*;
}

#[doc = include_str!("../Readme.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    #[cfg(feature = "rstest")]
    use rstest::rstest;

    #[cfg(feature = "rstest")]
    use crate::prelude::{test_app, TestApp};

    #[cfg(feature = "rstest")]
    #[rstest]
    fn with_rstest_fixtures(#[from(test_app)] mut app: TestApp) {
        // if it compiles, it's fine
        if skip_feature_test_body() {
            return;
        }

        use bevy_app::AppExit;

        use crate::prelude::{CollectEvents, SendEvents};

        app.collect_events::<AppExit>()
            .send_event_default::<AppExit>();
    }

    #[cfg(feature = "insta")]
    #[rstest]
    fn can_access_insta_macro() {
        // if it compiles, it's fine
        if skip_feature_test_body() {
            return;
        }

        set_snapshot_suffix!("works");
    }

    #[cfg(feature = "speculoos")]
    #[rstest]
    fn can_access_assertions() {
        // if it compiles, it's fine
        if skip_feature_test_body() {
            return;
        }

        use speculoos::assert_that;

        use crate::prelude::IsContainedIn;

        let items = vec![1, 2, 3];
        assert_that!(1).is_contained_in(&items);
    }

    #[cfg(feature = "iter_tools")]
    #[rstest]
    fn can_access_query_vec() {
        // if it compiles, it's fine
        if skip_feature_test_body() {
            return;
        }

        use bevy_app::App;
        use bevy_ecs::entity::Entity;

        use crate::{test_app::TestApp, traits::ImmediateQuery};

        TestApp(App::new()).query_vec::<Entity>();
    }

    #[allow(dead_code)]
    fn skip_feature_test_body() -> bool {
        // basically return true always but don't let the compiler know
        // so it checks code that we don't want to run anyway
        std::env::var("_SKIP_FEATURE_TESTS_")
            .map(|s| matches!(s.as_str(), "true" | "1"))
            .unwrap_or(true)
    }
}
