#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

// TODO: setup tags for different versions of bevy

#[allow(dead_code)]
mod deprecated {
    use konst::{iter, result, string};
    use static_assertions::const_assert_eq;
    const VERSION: [u32; 3] = iter::collect_const!(u32 =>
        string::split(env!("CARGO_PKG_VERSION"), "."),
        map(|s| result::unwrap!(u32::from_str_radix(s, 10))));
    const MAJOR: u32 = VERSION[0];
    const MINOR: u32 = VERSION[1];
    const PATCH: u32 = VERSION[2];
    const_assert_eq!(MAJOR, 0);
}

#[cfg(feature = "speculoos")]
pub(crate) mod assertions;
#[cfg(any(test, feature = "minimal"))]
pub(crate) mod fixtures;
#[cfg(any(all(test, feature = "rstest"), feature = "trait_collect_messages"))]
pub(crate) mod messages;
#[allow(unused_imports)] // Silence warning about name starting with `test_`
pub(crate) mod test_app;
pub(crate) mod traits;

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
    #[cfg(any(test, feature = "rstest"))]
    pub use super::fixtures::*;
    #[cfg(feature = "trait_collect_messages")]
    pub use super::messages::*;
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
    #[allow(unused_imports)]
    use rstest::rstest;

    #[cfg(feature = "rstest")]
    use crate::prelude::*;

    #[cfg(feature = "rstest")]
    #[rstest]
    fn with_rstest_fixtures(#[from(test_app)] mut app: TestApp) {
        // if it compiles, it's fine
        if skip_feature_test_body() {
            return;
        }

        use bevy_app::AppExit;

        use crate::prelude::*;

        app.collect_messages::<AppExit>()
            .write_message_default::<AppExit>();
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

        use crate::prelude::*;

        let items = vec![1, 2, 3];
        assert_that!(1).is_contained_in(&items);
    }

    #[ignore = "fix immediate_query.rs"]
    #[cfg(feature = "itertools")]
    #[rstest]
    fn can_access_query_vec() {
        // if it compiles, it's fine
        if skip_feature_test_body() {
            return;
        }

        // use bevy_app::App;
        // use bevy_ecs::entity::Entity;
        // use crate::{test_app::TestApp, traits::ImmediateQuery};
        // TestApp(App::new()).query_vec::<Entity>();
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
