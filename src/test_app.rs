use std::time::Duration;

use bevy::{
    app::{App, AppExit},
    ecs::{
        event::{Event, SendBatchIds},
        query::{QueryData, QueryFilter, QuerySingleError, ReadOnlyQueryData, WorldQuery},
    },
    prelude::{Deref, DerefMut, Time, Virtual},
};

#[derive(Debug, Deref, DerefMut)]
pub struct TestApp(pub App);

// NOTE: this is now also handled by feature `bevy/bevy_ci_testing`
impl Drop for TestApp {
    fn drop(&mut self) {
        self.world_mut().send_event(AppExit::Success);
    }
}
