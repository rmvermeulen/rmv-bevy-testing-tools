use bevy_ecs::message::{Message, WriteBatchIds};

use crate::prelude::TestApp;

#[doc = include_str!("./write_messages.md")]
pub trait WriteMessages {
    fn write_message_default<E: Message + Default>(&mut self);
    fn write_message<E: Message>(&mut self, message: E);
    fn write_message_batch<E: Message>(
        &mut self,
        messages: impl IntoIterator<Item = E>,
    ) -> Option<WriteBatchIds<E>>;
}

impl WriteMessages for TestApp {
    fn write_message_default<E: Message + Default>(&mut self) {
        self.world_mut().write_message_default::<E>();
    }
    fn write_message<E: Message>(&mut self, message: E) {
        self.world_mut().write_message::<E>(message);
    }
    fn write_message_batch<E: Message>(
        &mut self,
        messages: impl IntoIterator<Item = E>,
    ) -> Option<WriteBatchIds<E>> {
        self.world_mut().write_message_batch(messages)
    }
}
