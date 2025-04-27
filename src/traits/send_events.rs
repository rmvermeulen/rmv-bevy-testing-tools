use bevy::ecs::event::{Event, SendBatchIds};

use crate::prelude::TestApp;

pub trait SendEvents {
    fn send_event_default<E: Event + Default>(&mut self);
    fn send_event<E: Event>(&mut self, event: E);
    fn send_event_batch<E: Event>(
        &mut self,
        events: impl IntoIterator<Item = E>,
    ) -> Option<SendBatchIds<E>>;
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
