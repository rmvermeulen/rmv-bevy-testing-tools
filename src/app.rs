use std::time::Duration;

use bevy_app::{App, AppExit};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    event::{Event, SendBatchIds},
    query::{QueryData, QueryFilter, QuerySingleError, ReadOnlyQueryData, WorldQuery},
};
use bevy_internal::time::{Time, Virtual};
use bevy_state::state::{FreelyMutableState, NextState, State, States};

use crate::{
    events::{CollectedEvents, EventCollectorPlugin, EventFilterPlugin},
    traits::{AdvanceTime, BasicQuery, CollectEvents, ImmediateQuery, ManageState, SendEvents},
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

impl ManageState for TestApp {
    fn get_state<S: States>(&self) -> Option<&S> {
        self.world().get_resource::<State<S>>().map(|s| s.get())
    }
    fn get_next_state<S: FreelyMutableState>(&self) -> Option<&NextState<S>> {
        self.world().get_resource::<NextState<S>>()
    }
    fn set_next_state<S: FreelyMutableState>(&mut self, next: S) -> Option<()> {
        self.world_mut()
            .get_resource_mut::<NextState<S>>()
            .map(|mut s| s.set(next))
    }
}

impl BasicQuery for TestApp {
    fn query_any<'a, Q, C>(&mut self) -> bool
    where
        Q: QueryData<Item<'a> = C>,
    {
        let mut q = self.world_mut().query::<Q>();
        q.iter(self.world()).next().is_some()
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
        use iter_tools::Itertools;

        let mut query = self.world_mut().query::<D>();
        query.iter(self.world_mut()).collect_vec()
    }

    fn query_collect<D, C>(&mut self) -> C
    where
        D: ReadOnlyQueryData,
        for<'a> C: std::iter::FromIterator<<D as bevy_ecs::query::WorldQuery>::Item<'a>>,
    {
        let mut query = self.world_mut().query::<D>();
        let result = query.iter(self.world_mut()).collect::<C>();
        result
    }
}

impl AdvanceTime for TestApp {
    fn advance_time_by(app: &mut TestApp, duration: Duration) {
        app.world_mut()
            .get_resource_mut::<Time<Virtual>>()
            .unwrap()
            .advance_by(duration);
    }
    fn advance_time_to(app: &mut TestApp, duration: Duration) {
        app.world_mut()
            .get_resource_mut::<Time<Virtual>>()
            .unwrap()
            .advance_to(duration);
    }
}
