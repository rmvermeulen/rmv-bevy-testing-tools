use std::marker::PhantomData;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    event::{Event, EventReader},
    resource::Resource,
    system::ResMut,
};

#[derive(Debug, Deref, DerefMut, Resource)]
pub struct CollectedEvents<E>(Vec<E>);

impl<E: Event> CollectedEvents<E> {
    pub fn get(&self) -> &Vec<E> {
        &self.0
    }
}

impl<E: Event> Default for CollectedEvents<E> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug)]
pub struct EventCollectorPlugin<E>(PhantomData<E>)
where
    E: Event + Clone;

impl<E: Event + Clone> Default for EventCollectorPlugin<E> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<E: Event + Clone> Plugin for EventCollectorPlugin<E> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn build(&self, app: &mut App) {
        app.add_event::<E>()
            .init_resource::<CollectedEvents<E>>()
            .add_systems(
                PostUpdate,
                |mut events: EventReader<E>, mut collection: ResMut<CollectedEvents<E>>| {
                    collection.extend(events.read().cloned());
                },
            );
    }
}

#[derive(Debug)]
pub enum EventFilterPlugin<E>
where
    E: Event + Clone + PartialEq,
{
    Only(E),
    AnyOf(Vec<E>),
}

impl<E: Event + Clone + PartialEq> Plugin for EventFilterPlugin<E> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn build(&self, app: &mut App) {
        app.add_event::<E>().init_resource::<CollectedEvents<E>>();
        match &self {
            EventFilterPlugin::Only(event) => {
                app.add_systems(PostUpdate, {
                    let event = event.clone();
                    move |mut events: EventReader<E>, mut collection: ResMut<CollectedEvents<E>>| {
                        collection.extend(events.read().filter(|ev| *ev == &event).cloned());
                    }
                });
            }
            EventFilterPlugin::AnyOf(ref any_of_events) => {
                app.add_systems(PostUpdate, {
                    let any_of_events = any_of_events.clone();
                    move |mut events: EventReader<E>, mut collection: ResMut<CollectedEvents<E>>| {
                        collection.extend(
                            events
                                .read()
                                .filter(|ev| any_of_events.contains(ev))
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
    use bevy_ecs::event::EventWriter;
    use rstest::*;
    use speculoos::prelude::*;

    use super::*;
    use crate::{fixtures::minimal_test_app, test_app::TestApp, traits::CollectEvents};

    #[rstest]
    fn test_collected_events_default_deref() {
        let collected_events: CollectedEvents<CmpEvent> = CollectedEvents::default();
        let v1: &Vec<_> = &*collected_events;
        let v2: &Vec<_> = collected_events.get();
        assert_that!(v1).is_equal_to(v2);
    }

    #[derive(Clone, Copy, Debug, Event)]
    struct NonEqEvent;

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(10)]
    fn test_event_collector_plugin(
        #[from(minimal_test_app)]
        #[with(EventCollectorPlugin::<NonEqEvent>::default())]
        mut app: TestApp,
        #[case] emit_count: usize,
    ) {
        use crate::traits::CollectEvents;

        app.add_systems(Update, move |mut writer: EventWriter<NonEqEvent>| {
            for _ in 0..emit_count {
                writer.write(NonEqEvent);
            }
        });

        app.update();

        assert_that!(app.get_collected_events::<NonEqEvent>())
            .is_some()
            .has_length(emit_count);
    }

    #[derive(Clone, Debug, Event, PartialEq)]
    enum CmpEvent {
        A,
        B,
        C,
    }

    #[rstest]
    #[case("ABCA", "A", "AA")]
    #[case("BCAB", "B", "BB")]
    #[case("CABC", "C", "CC")]
    fn test_event_filter_plugin_only(
        #[case] events_to_emit: EventList<CmpEvent>,
        #[case] only_event: CmpEvent,
        #[case] expected_events: EventList<CmpEvent>,
        #[from(minimal_test_app)]
        #[with(EventFilterPlugin::Only(only_event.clone()))]
        mut app: TestApp,
    ) {
        app.add_systems(Update, move |mut writer: EventWriter<CmpEvent>| {
            for e in &*events_to_emit {
                writer.write(e.clone());
            }
        });

        app.update();

        let collected_events = app.get_collected_events::<CmpEvent>();
        assert_that!(collected_events)
            .is_some()
            .is_equal_to(&*expected_events);

        for e in &collected_events.unwrap() {
            assert_that!(e).is_equal_to(&only_event);
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
    fn test_event_filter_plugin_any_of(
        #[case] events_to_emit: EventList<CmpEvent>,
        #[case] any_of_events: EventList<CmpEvent>,
        #[case] expected_events: EventList<CmpEvent>,
        #[from(minimal_test_app)]
        #[with(EventFilterPlugin::AnyOf((*any_of_events).clone()))]
        mut app: TestApp,
    ) {
        use crate::traits::CollectEvents;

        app.add_systems(Update, move |mut writer: EventWriter<CmpEvent>| {
            for e in &*events_to_emit {
                writer.write(e.clone());
            }
        });

        app.update();

        let collected_events = app.get_collected_events::<CmpEvent>();
        assert_that!(collected_events)
            .is_some()
            .is_equal_to(&*expected_events);

        for e in collected_events.unwrap().into_iter() {
            assert_that!(*any_of_events).contains(e);
        }
    }

    pub struct InvalidEvent;

    impl FromStr for CmpEvent {
        type Err = InvalidEvent;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(CmpEvent::A),
                "B" => Ok(CmpEvent::B),
                "C" => Ok(CmpEvent::C),
                _ => Err(InvalidEvent),
            }
        }
    }

    #[rstest]
    #[case("A", Some(CmpEvent::A))]
    #[case("B", Some(CmpEvent::B))]
    #[case("C", Some(CmpEvent::C))]
    #[should_panic]
    #[case("", None)]
    #[should_panic]
    #[case("D", None)]
    #[should_panic]
    #[case("more nonsense", None)]
    fn test_filtered_event_fromstr(#[case] magic: CmpEvent, #[case] expected: Option<CmpEvent>) {
        assert_that!(magic).is_equal_to(expected.unwrap());
    }

    #[derive(Clone, Debug, Deref)]
    struct EventList<E: Event + Clone>(Vec<E>);

    impl<E: Event + Clone + FromStr<Err = InvalidEvent>> FromStr for EventList<E> {
        type Err = InvalidEvent;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut events = Vec::new();
            for c in s.chars() {
                let e = E::from_str(&c.to_string())?;
                events.push(e);
            }
            Ok(EventList(events))
        }
    }

    #[rstest]
    #[case("A", vec![CmpEvent::A])]
    #[case("AB", vec![CmpEvent::A, CmpEvent::B])]
    #[case("ABC", vec![CmpEvent::A, CmpEvent::B, CmpEvent::C])]
    #[case("AABBCC", vec![
        CmpEvent::A, CmpEvent::A,
        CmpEvent::B, CmpEvent::B,
        CmpEvent::C, CmpEvent::C
    ])]
    #[should_panic]
    #[case("abc", vec![])]
    fn test_event_list_fromstr(
        #[case] magic: EventList<CmpEvent>,
        #[case] expected: Vec<CmpEvent>,
    ) {
        assert_that!(*magic).is_equal_to(&expected);
    }
}
