use bevy_ecs::message::Message;

use crate::{
    messages::{CollectedMessages, MessageCollectorPlugin, MessageFilterPlugin},
    prelude::TestApp,
};

#[doc = include_str!("./collect_messages.md")]
pub trait CollectMessages {
    fn collect_messages<E: Message + Clone>(&mut self) -> &mut Self;
    fn collect_messages_only<E: Message + Clone + PartialEq>(&mut self, message: E) -> &mut Self;
    fn collect_messages_any_of<E: Message + Clone + PartialEq>(&mut self, messages: &[E]) -> &mut Self;
    fn get_collected_messages<E: Message + Clone>(&self) -> Option<Vec<E>>;
}

impl CollectMessages for TestApp {
    fn collect_messages<E: Message + Clone>(&mut self) -> &mut TestApp {
        self.add_plugins(MessageCollectorPlugin::<E>::default());
        self
    }

    fn collect_messages_only<E: Message + Clone + PartialEq>(&mut self, message: E) -> &mut TestApp {
        self.add_plugins(MessageFilterPlugin::<E>::Only(message.clone()));
        self
    }

    fn collect_messages_any_of<E: Message + Clone + PartialEq>(&mut self, messages: &[E]) -> &mut Self {
        self.add_plugins(MessageFilterPlugin::<E>::AnyOf(messages.into()));
        self
    }

    fn get_collected_messages<E: Message + Clone>(&self) -> Option<Vec<E>> {
        self.world()
            .get_resource::<CollectedMessages<E>>()
            .map(|e| e.get().clone())
    }
}

#[cfg(test)]
mod tests {

    use bevy_ecs::message::Message;
    use rstest::rstest;
    use speculoos::{assert_that, option::OptionAssertions};

    use super::*;
    use crate::{messages::CollectedMessages, fixtures::minimal_test_app, test_app::TestApp};

    #[derive(Message, Clone, Debug, PartialEq)]
    struct MyMessage;

    #[rstest]
    fn message_collector_trait_get_collected_messages(#[from(minimal_test_app)] mut app: TestApp) {
        assert_that!(app.get_collected_messages::<MyMessage>())
            .named("no resource")
            .is_none();

        app.insert_resource(CollectedMessages::<MyMessage>::default());

        assert_that!(app.get_collected_messages::<MyMessage>())
            .named("after resource inserted")
            .is_some();
    }

    #[rstest]
    fn message_collector_trait_collect_messages(#[from(minimal_test_app)] mut app: TestApp) {
        assert_that!(app.get_collected_messages::<MyMessage>())
            .named("no plugin before collect_messages")
            .is_none();

        app.collect_messages::<MyMessage>();

        assert_that!(app.get_collected_messages::<MyMessage>())
            .named("plugin inserted after collect_messages")
            .is_some();
    }

    #[rstest]
    fn message_collector_trait_collect_messages_only(#[from(minimal_test_app)] mut app: TestApp) {
        assert_that!(app.get_collected_messages::<MyMessage>())
            .named("no plugin before collect_messages_only")
            .is_none();

        app.collect_messages_only(MyMessage);

        assert_that!(app.get_collected_messages::<MyMessage>())
            .named("plugin inserted after collect_messages_only")
            .is_some();
    }

    #[rstest]
    fn message_collector_trait_collect_messages_any_of(#[from(minimal_test_app)] mut app: TestApp) {
        assert_that!(app.get_collected_messages::<MyMessage>())
            .named("no plugin before collect_messages_any_of")
            .is_none();

        app.collect_messages_any_of(&[MyMessage]);

        assert_that!(app.get_collected_messages::<MyMessage>())
            .named("plugin inserted after collect_messages_any_of")
            .is_some();
    }
}
