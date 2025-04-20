use super::*;
use crate::{
    events::{CollectedEvents, EventCollectorPlugin, EventFilterPlugin},
    prelude::TestApp,
};

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
