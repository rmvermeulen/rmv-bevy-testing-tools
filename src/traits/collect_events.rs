use bevy_ecs::event::Event;

use crate::{
    events::{CollectedEvents, EventCollectorPlugin, EventFilterPlugin},
    prelude::TestApp,
};

#[doc = include_str!("./collect_events.md")]
pub trait CollectEvents {
    fn collect_events<E: Event + Clone>(&mut self) -> &mut Self;
    fn collect_events_only<E: Event + Clone + PartialEq>(&mut self, event: E) -> &mut Self;
    fn collect_events_any_of<E: Event + Clone + PartialEq>(&mut self, events: &[E]) -> &mut Self;
    fn get_collected_events<E: Event + Clone>(&self) -> Option<Vec<E>>;
}

impl CollectEvents for TestApp {
    fn collect_events<E: Event + Clone>(&mut self) -> &mut TestApp {
        self.add_plugins(EventCollectorPlugin::<E>::default());
        self
    }

    fn collect_events_only<E: Event + Clone + PartialEq>(&mut self, event: E) -> &mut TestApp {
        self.add_plugins(EventFilterPlugin::<E>::Only(event.clone()));
        self
    }

    fn collect_events_any_of<E: Event + Clone + PartialEq>(&mut self, events: &[E]) -> &mut Self {
        self.add_plugins(EventFilterPlugin::<E>::AnyOf(events.into()));
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

    use bevy_ecs::event::Event;
    use rstest::rstest;
    use speculoos::{assert_that, option::OptionAssertions};

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
}
