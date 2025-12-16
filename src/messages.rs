use std::marker::PhantomData;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    message::{Message, MessageReader},
    resource::Resource,
    system::ResMut,
};

#[derive(Debug, Deref, DerefMut, Resource)]
pub struct CollectedMessages<E>(Vec<E>);

impl<E: Message> CollectedMessages<E> {
    pub fn get(&self) -> &Vec<E> {
        &self.0
    }
}

impl<E: Message> Default for CollectedMessages<E> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug)]
pub struct MessageCollectorPlugin<E>(PhantomData<E>)
where
    E: Message + Clone;

impl<E: Message + Clone> Default for MessageCollectorPlugin<E> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<E: Message + Clone> Plugin for MessageCollectorPlugin<E> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn build(&self, app: &mut App) {
        app.add_message::<E>()
            .init_resource::<CollectedMessages<E>>()
            .add_systems(
                PostUpdate,
                |mut messages: MessageReader<E>, mut collection: ResMut<CollectedMessages<E>>| {
                    collection.extend(messages.read().cloned());
                },
            );
    }
}

#[derive(Debug)]
pub enum MessageFilterPlugin<E>
where
    E: Message + Clone + PartialEq,
{
    Only(E),
    AnyOf(Vec<E>),
}

impl<E: Message + Clone + PartialEq> Plugin for MessageFilterPlugin<E> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn build(&self, app: &mut App) {
        app.add_message::<E>()
            .init_resource::<CollectedMessages<E>>();
        match &self {
            MessageFilterPlugin::Only(message) => {
                app.add_systems(PostUpdate, {
                    let message = message.clone();
                    move |mut messages: MessageReader<E>,
                          mut collection: ResMut<CollectedMessages<E>>| {
                        collection.extend(messages.read().filter(|ev| *ev == &message).cloned());
                    }
                });
            }
            MessageFilterPlugin::AnyOf(any_of_messages) => {
                app.add_systems(PostUpdate, {
                    let any_of_messages = any_of_messages.clone();
                    move |mut messages: MessageReader<E>,
                          mut collection: ResMut<CollectedMessages<E>>| {
                        collection.extend(
                            messages
                                .read()
                                .filter(|ev| any_of_messages.contains(ev))
                                .cloned(),
                        );
                    }
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bevy_app::Update;
    use bevy_ecs::message::MessageWriter;
    use rstest::*;
    use speculoos::prelude::*;

    use super::*;
    use crate::{fixtures::minimal_test_app, test_app::TestApp, traits::CollectMessages};

    #[rstest]
    fn test_collected_messages_default_deref() {
        let collected_messages: CollectedMessages<CmpMessage> = CollectedMessages::default();
        let v1: &Vec<_> = &*collected_messages;
        let v2: &Vec<_> = collected_messages.get();
        assert_that!(v1).is_equal_to(v2);
    }

    #[derive(Clone, Copy, Debug, Message)]
    struct NonEqMessage;

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(10)]
    fn test_message_collector_plugin(
        #[from(minimal_test_app)]
        #[with(MessageCollectorPlugin::<NonEqMessage>::default())]
        mut app: TestApp,
        #[case] emit_count: usize,
    ) {
        use crate::traits::CollectMessages;

        app.add_systems(Update, move |mut writer: MessageWriter<NonEqMessage>| {
            for _ in 0..emit_count {
                writer.write(NonEqMessage);
            }
        });

        app.update();

        assert_that!(app.get_collected_messages::<NonEqMessage>())
            .is_some()
            .has_length(emit_count);
    }

    #[derive(Clone, Debug, Message, PartialEq)]
    enum CmpMessage {
        A,
        B,
        C,
    }

    #[rstest]
    #[case("ABCA", "A", "AA")]
    #[case("BCAB", "B", "BB")]
    #[case("CABC", "C", "CC")]
    fn test_message_filter_plugin_only(
        #[case] messages_to_emit: MessageList<CmpMessage>,
        #[case] only_message: CmpMessage,
        #[case] expected_messages: MessageList<CmpMessage>,
        #[from(minimal_test_app)]
        #[with(MessageFilterPlugin::Only(only_message.clone()))]
        mut app: TestApp,
    ) {
        app.add_systems(Update, move |mut writer: MessageWriter<CmpMessage>| {
            for e in &*messages_to_emit {
                writer.write(e.clone());
            }
        });

        app.update();

        let collected_messages = app.get_collected_messages::<CmpMessage>();
        assert_that!(collected_messages)
            .is_some()
            .is_equal_to(&*expected_messages);

        for e in &collected_messages.unwrap() {
            assert_that!(e).is_equal_to(&only_message);
        }
    }

    #[rstest]
    #[case("AABBCC", "A", "AA")]
    #[case("AABBCC", "B", "BB")]
    #[case("AABBCC", "C", "CC")]
    #[case("ABCCBA", "AB", "ABBA")]
    #[case("ABCCBA", "AC", "ACCA")]
    #[case("ABCCBA", "BC", "BCCB")]
    #[case("AABBCC", "ABC", "AABBCC")]
    fn test_message_filter_plugin_any_of(
        #[case] messages_to_emit: MessageList<CmpMessage>,
        #[case] any_of_messages: MessageList<CmpMessage>,
        #[case] expected_messages: MessageList<CmpMessage>,
        #[from(minimal_test_app)]
        #[with(MessageFilterPlugin::AnyOf((*any_of_messages).clone()))]
        mut app: TestApp,
    ) {
        use crate::traits::CollectMessages;

        app.add_systems(Update, move |mut writer: MessageWriter<CmpMessage>| {
            for e in &*messages_to_emit {
                writer.write(e.clone());
            }
        });

        app.update();

        let collected_messages = app.get_collected_messages::<CmpMessage>();
        assert_that!(collected_messages)
            .is_some()
            .is_equal_to(&*expected_messages);

        for e in collected_messages.unwrap().into_iter() {
            assert_that!(*any_of_messages).contains(e);
        }
    }

    pub struct InvalidMessage;

    impl FromStr for CmpMessage {
        type Err = InvalidMessage;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(CmpMessage::A),
                "B" => Ok(CmpMessage::B),
                "C" => Ok(CmpMessage::C),
                _ => Err(InvalidMessage),
            }
        }
    }

    #[rstest]
    #[case("A", Some(CmpMessage::A))]
    #[case("B", Some(CmpMessage::B))]
    #[case("C", Some(CmpMessage::C))]
    #[should_panic]
    #[case("", None)]
    #[should_panic]
    #[case("D", None)]
    #[should_panic]
    #[case("more nonsense", None)]
    fn test_filtered_message_fromstr(
        #[case] magic: CmpMessage,
        #[case] expected: Option<CmpMessage>,
    ) {
        assert_that!(magic).is_equal_to(expected.unwrap());
    }

    #[derive(Clone, Debug, Deref)]
    struct MessageList<E: Message + Clone>(Vec<E>);

    impl<E: Message + Clone + FromStr<Err = InvalidMessage>> FromStr for MessageList<E> {
        type Err = InvalidMessage;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut messages = Vec::new();
            for c in s.chars() {
                let e = E::from_str(&c.to_string())?;
                messages.push(e);
            }
            Ok(MessageList(messages))
        }
    }

    #[rstest]
    #[case("A", vec![CmpMessage::A])]
    #[case("AB", vec![CmpMessage::A, CmpMessage::B])]
    #[case("ABC", vec![CmpMessage::A, CmpMessage::B, CmpMessage::C])]
    #[case("AABBCC", vec![
        CmpMessage::A, CmpMessage::A,
        CmpMessage::B, CmpMessage::B,
        CmpMessage::C, CmpMessage::C
    ])]
    #[should_panic]
    #[case("abc", vec![])]
    fn test_message_list_fromstr(
        #[case] magic: MessageList<CmpMessage>,
        #[case] expected: Vec<CmpMessage>,
    ) {
        assert_that!(*magic).is_equal_to(&expected);
    }
}
