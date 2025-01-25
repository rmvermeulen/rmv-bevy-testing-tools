use bevy_app::{App, AppExit};
use bevy_derive::{Deref, DerefMut};
use bevy_state::state::{FreelyMutableState, NextState, State, States};

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
