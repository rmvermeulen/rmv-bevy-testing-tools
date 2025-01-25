use bevy_app::{App, AppExit};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::event::{Event, SendBatchIds};
use bevy_state::state::{FreelyMutableState, NextState, State, States};

use crate::{
    events::{CollectedEvents, EventCollectorPlugin, EventFilterPlugin},
    traits::{CollectEvents, SendEvents},
};

#[derive(Debug, Deref, DerefMut)]
pub struct TestApp(pub App);

impl TestApp {
    pub fn get_state<S: States>(&self) -> Option<&S> {
        self.world().get_resource::<State<S>>().map(|s| s.get())
    }
    pub fn get_next_state<S: FreelyMutableState>(&self) -> Option<&NextState<S>> {
        self.world().get_resource::<NextState<S>>()
    }
    pub fn set_next_state<S: FreelyMutableState>(&mut self, next: S) -> Option<()> {
        self.world_mut()
            .get_resource_mut::<NextState<S>>()
            .map(|mut s| s.set(next))
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        self.world_mut().send_event(AppExit::Success);
    }
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

impl SendEvents for TestApp {
    fn send_event_default<E: Event + Default>(&mut self) {
        self.world_mut().send_event_default::<E>();
    }
    fn send_event<E: Event>(&mut self, event: E) {
        self.world_mut().send_event::<E>(event);
    }
    fn send_event_batch<E: Event>(
        &mut self,
        events: impl IntoIterator<Item = E>,
    ) -> Option<SendBatchIds<E>> {
        self.world_mut().send_event_batch(events)
    }
}
