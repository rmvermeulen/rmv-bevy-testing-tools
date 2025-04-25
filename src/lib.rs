#![feature(coverage_attribute)]

// TODO: setup tags for different versions of bevy

#[cfg(feature = "speculoos")]
pub mod assertions;
pub mod events;
#[cfg(any(test, feature = "rstest"))]
pub mod fixtures;
#[allow(unused_imports)]
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
    #[cfg(feature = "rstest")]
    pub use super::fixtures::*;
    #[cfg(feature = "insta")]
    pub use super::set_snapshot_suffix;
    pub use super::{events::*, test_app, test_app::*, traits::*};
}

#[doc = include_str!("../Readme.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use bevy_ecs::event::Event;
    use bevy_state::state::States;
    #[cfg(feature = "rstest")]
    use rstest::rstest;

    #[derive(Event, Default, Debug, Copy, Clone)]
    struct MyEvent;
    #[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
    enum MyState {
        #[default]
        A,
        B,
    }

    #[cfg(feature = "rstest")]
    #[rstest]
    fn can_access_everything(#[from(test_app)] mut app: TestApp) {
        if do_not_run() {
            return;
        }
        use bevy_state::app::AppExtStates;
        use rstest::rstest;

        use crate::test_app;

        app.collect_events::<MyEvent>()
            .send_event_default::<MyEvent>();

        app.init_state::<MyState>();
        app.set_next_state(MyState::B);
    }

    #[cfg(feature = "insta")]
    #[rstest]
    fn can_access_insta_macro() {
        if do_not_run() {
            return;
        }
        set_snapshot_suffix!("works");
    }

    #[cfg(feature = "speculoos")]
    #[rstest]
    fn can_access_assertions() {
        if do_not_run() {
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
        if do_not_run() {
            return;
        }

        use bevy_app::App;
        use bevy_ecs::entity::Entity;

        TestApp(App::new()).query_vec::<Entity>();
    }

    fn do_not_run() -> bool {
        // basically return true always but don't let the compiler know
        std::env::var("_SKIP_FEATURE_TESTS_")
            .map(|s| matches!(s.as_str(), "true" | "1"))
            .unwrap_or(true)
    }
}
