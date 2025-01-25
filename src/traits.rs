use bevy_ecs::{
    event::{Event, SendBatchIds},
    query::{QueryFilter, QuerySingleError, ReadOnlyQueryData, WorldQuery},
};
#[cfg(feature = "iter_tools")]
use iter_tools::Itertools;

use crate::app::TestApp;

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
    fn query_collect<D>(&mut self) -> Vec<<D as WorldQuery>::Item<'_>>
    where
        D: ReadOnlyQueryData;
    #[cfg(feature = "iter_tools")]
    fn query_vec<D>(&mut self) -> Vec<<D as WorldQuery>::Item<'_>>
    where
        D: ReadOnlyQueryData;
}

impl ImmediateQuery for TestApp {
    fn query_single<D>(&mut self) -> Result<<D as WorldQuery>::Item<'_>, QuerySingleError>
    where
        D: ReadOnlyQueryData,
    {
        let mut query = self.world_mut().query::<D>();
        query.get_single(self.world_mut())
    }
    fn query_single_filtered<D, F>(
        &mut self,
    ) -> Result<<D as WorldQuery>::Item<'_>, QuerySingleError>
    where
        D: ReadOnlyQueryData,
        F: QueryFilter,
    {
        let mut query = self.world_mut().query_filtered::<D, F>();
        query.get_single(self.world_mut())
    }
    #[cfg(feature = "iter_tools")]
    fn query_vec<D>(&mut self) -> Vec<<D as WorldQuery>::Item<'_>>
    where
        D: ReadOnlyQueryData,
    {
        let mut query = self.world_mut().query::<D>();
        query.iter(self.world_mut()).collect_vec()
    }

    fn query_collect<D>(&mut self) -> Vec<<D as WorldQuery>::Item<'_>>
    where
        D: ReadOnlyQueryData,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use speculoos::{assert_that, option::OptionAssertions};

    use super::*;
    use crate::{events::CollectedEvents, fixtures::minimal_test_app};
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
