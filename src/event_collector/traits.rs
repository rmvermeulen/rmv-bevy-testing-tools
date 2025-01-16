use super::*;
pub trait EventCollectorTestApp {
    fn collect_events<E: Event + Clone>(&mut self) -> &mut Self;

    fn collect_events_only<E: Event + Clone + PartialEq>(&mut self, event: E) -> &mut Self;

    fn collect_events_any_of<E: Event + Clone + PartialEq>(&mut self, events: &[E]) -> &mut Self;

    fn get_collected_events<E: Event + Clone>(&self) -> Option<Vec<E>>;
}

impl EventCollectorTestApp for TestApp {
    fn collect_events<E: Event + Clone>(&mut self) -> &mut TestApp {
        self.add_plugins(EventCollector::<E>::default());
        self
    }

    fn collect_events_only<E: Event + Clone + PartialEq>(&mut self, event: E) -> &mut TestApp {
        self.add_plugins(EventFilter::<E>::Only(event.clone()));
        self
    }

    fn collect_events_any_of<E: Event + Clone + PartialEq>(&mut self, events: &[E]) -> &mut Self {
        self.add_plugins(EventFilter::<E>::AnyOf(events.into()));
        self
    }

    fn get_collected_events<E: Event + Clone>(&self) -> Option<Vec<E>> {
        self.world()
            .get_resource::<CollectedEvents<E>>()
            .map(|e| e.get().clone())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use speculoos::{assert_that, option::OptionAssertions};

    use crate::minimal_test_app;

    use super::*;
    #[derive(Event, Clone, Debug, PartialEq)]
    struct MyEvent;

    #[rstest]
    fn event_collector_trait_get_collected_events(#[from(minimal_test_app)] mut app: TestApp) {
        assert_that!(app.get_collected_events::<MyEvent>())
            .named("no resource")
            .is_none();

        app.insert_resource(CollectedEvents(vec![MyEvent, MyEvent]));

        assert_that!(app.get_collected_events::<MyEvent>())
            .named("after resource inserted")
            .is_some()
            .is_equal_to(vec![MyEvent, MyEvent]);
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
