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
    traits::{BasicQuery, CollectEvents, ImmediateQuery, ManageState, SendEvents, TimeControls},
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

// NOTE: this is now also handled by feature `bevy/bevy_ci_testing`
impl Drop for TestApp {
    fn drop(&mut self) {
        self.world_mut().send_event(AppExit::Success);
    }
}
