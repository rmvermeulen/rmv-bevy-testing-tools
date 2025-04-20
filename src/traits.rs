use bevy_ecs::event::Event;

mod basic_query;
mod collect_events;
mod immediate_query;
mod manage_state;
mod send_events;
mod time_controls;

pub use basic_query::*;
pub use collect_events::*;
pub use immediate_query::*;
pub use manage_state::*;
pub use send_events::*;
pub use time_controls::*;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy_internal::time::Time;
    use rstest::rstest;
    use speculoos::{assert_that, option::OptionAssertions, prelude::BooleanAssertions};

    use super::*;
    use crate::{events::CollectedEvents, fixtures::minimal_test_app, test_app::TestApp};

    #[derive(Event, Clone, Debug, PartialEq)]
    struct MyEvent;

    #[rstest]
    fn event_collector_trait_get_collected_events(#[from(minimal_test_app)] mut app: TestApp) {
        assert_that!(app.get_collected_events::<MyEvent>())
            .named("no resource")
            .is_none();

        app.insert_resource(CollectedEvents::<MyEvent>::default());

        assert_that!(app.get_collected_events::<MyEvent>())
            .named("after resource inserted")
            .is_some();
    }

    #[rstest]
    fn event_collector_trait_collect_events(#[from(minimal_test_app)] mut app: TestApp) {
        assert_that!(app.get_collected_events::<MyEvent>())
            .named("no plugin before collect_events")
            .is_none();

        app.collect_events::<MyEvent>();

        assert_that!(app.get_collected_events::<MyEvent>())
            .named("plugin inserted after collect_events")
            .is_some();
    }

    #[rstest]
    fn event_collector_trait_collect_events_only(#[from(minimal_test_app)] mut app: TestApp) {
        assert_that!(app.get_collected_events::<MyEvent>())
            .named("no plugin before collect_events_only")
            .is_none();

        app.collect_events_only(MyEvent);

        assert_that!(app.get_collected_events::<MyEvent>())
            .named("plugin inserted after collect_events_only")
            .is_some();
    }

    #[rstest]
    fn event_collector_trait_collect_events_any_of(#[from(minimal_test_app)] mut app: TestApp) {
        assert_that!(app.get_collected_events::<MyEvent>())
            .named("no plugin before collect_events_any_of")
            .is_none();

        app.collect_events_any_of(&[MyEvent]);

        assert_that!(app.get_collected_events::<MyEvent>())
            .named("plugin inserted after collect_events_any_of")
            .is_some();
    }

    #[rstest]
    fn pause(#[from(minimal_test_app)] mut app: TestApp) {
        app.pause();
        assert_that!(app.is_paused()).is_true();
    }

    #[rstest]
    fn unpause(#[from(minimal_test_app)] mut app: TestApp) {
        app.unpause();
        assert_that!(app.is_paused()).is_false();
    }

    #[rstest]
    #[case(0.0)]
    #[case(5.3)]
    #[case(123.3)]
    fn advance_time_to(#[from(minimal_test_app)] mut app: TestApp, #[case] seconds: f32) {
        app.pause();
        app.advance_time_to(Duration::from_secs_f32(seconds));
        assert_that!(app.world().get_resource::<Time>().map(|t| t.elapsed_secs()))
            .is_some()
            .is_equal_to(seconds);
    }

    #[rstest]
    #[case(0.0)]
    #[case(5.3)]
    #[case(123.3)]
    fn advance_time_by(#[from(minimal_test_app)] mut app: TestApp, #[case] seconds: f32) {
        app.pause();
        app.advance_time_by(Duration::from_secs_f32(3.0));
        app.advance_time_by(Duration::from_secs_f32(seconds));
        assert_that!(app.world().get_resource::<Time>().map(|t| t.elapsed_secs()))
            .is_some()
            .is_equal_to(3.0 + seconds);
    }
}
