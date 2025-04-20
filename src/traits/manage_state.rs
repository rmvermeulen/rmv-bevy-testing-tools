use bevy_state::state::{FreelyMutableState, NextState, State, States};

use crate::prelude::TestApp;

pub trait ManageState {
    fn get_state<S: States>(&self) -> Option<&S>;
    fn get_next_state<S: FreelyMutableState>(&self) -> Option<&NextState<S>>;
    fn set_next_state<S: FreelyMutableState>(&mut self, next: S) -> Option<()>;
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
