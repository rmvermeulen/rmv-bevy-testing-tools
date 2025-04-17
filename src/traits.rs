use std::time::Duration;

use bevy_ecs::{
    event::{Event, SendBatchIds},
    query::{QueryData, QueryFilter, QuerySingleError, ReadOnlyQueryData, WorldQuery},
};
use bevy_state::state::{FreelyMutableState, NextState, States};

pub trait CollectEvents {
    fn collect_events<E: Event + Clone>(&mut self) -> &mut Self;

    fn collect_events_only<E: Event + Clone + PartialEq>(&mut self, event: E) -> &mut Self;

    fn collect_events_any_of<E: Event + Clone + PartialEq>(&mut self, events: &[E]) -> &mut Self;

    fn get_collected_events<E: Event + Clone>(&self) -> Option<Vec<E>>;
}

pub trait SendEvents {
    fn send_event_default<E: Event + Default>(&mut self);
    fn send_event<E: Event>(&mut self, event: E);
    fn send_event_batch<E: Event>(
        &mut self,
        events: impl IntoIterator<Item = E>,
    ) -> Option<SendBatchIds<E>>;
}

pub trait ManageState {
    fn get_state<S: States>(&self) -> Option<&S>;
    fn get_next_state<S: FreelyMutableState>(&self) -> Option<&NextState<S>>;
    fn set_next_state<S: FreelyMutableState>(&mut self, next: S) -> Option<()>;
}

pub trait BasicQuery {
    fn query_any<'a, Q, C>(&mut self) -> bool
    where
        Q: QueryData<Item<'a> = C>;
}

pub trait ImmediateQuery {
    fn query_single<D>(&mut self) -> Result<<D as WorldQuery>::Item<'_>, QuerySingleError>
    where
        D: ReadOnlyQueryData;
    fn query_single_filtered<D, F>(
        &mut self,
    ) -> Result<<D as WorldQuery>::Item<'_>, QuerySingleError>
    where
        D: ReadOnlyQueryData,
        F: QueryFilter;
    fn query_collect<D, C>(&mut self) -> C
    where
        D: ReadOnlyQueryData,
        for<'a> C: std::iter::FromIterator<<D as bevy_ecs::query::WorldQuery>::Item<'a>>;
    #[cfg(feature = "iter_tools")]
    fn query_vec<D>(&mut self) -> Vec<<D as WorldQuery>::Item<'_>>
    where
        D: ReadOnlyQueryData;
}

pub trait AdvanceTime {
    fn advance_time_to(app: &mut Self, duration: Duration);
    fn advance_time_by(app: &mut Self, duration: Duration);
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use speculoos::{assert_that, option::OptionAssertions};

    use super::*;
    use crate::{app::TestApp, events::CollectedEvents, fixtures::minimal_test_app};
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
}
