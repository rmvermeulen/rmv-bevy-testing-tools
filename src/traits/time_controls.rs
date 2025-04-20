use std::time::Duration;

use bevy_internal::time::{Time, Virtual};

use crate::prelude::TestApp;

pub trait TimeControls {
    fn is_paused(&self) -> bool;
    fn pause(&mut self);
    fn unpause(&mut self);
    fn advance_time_to(&mut self, duration: Duration);
    fn advance_time_by(&mut self, duration: Duration);
}

impl TimeControls for TestApp {
    fn is_paused(&self) -> bool {
        self.world()
            .get_resource::<Time<Virtual>>()
            .unwrap()
            .is_paused()
    }
    fn pause(&mut self) {
        self.world_mut()
            .get_resource_mut::<Time<Virtual>>()
            .unwrap()
            .pause()
    }
    fn unpause(&mut self) {
        self.world_mut()
            .get_resource_mut::<Time<Virtual>>()
            .unwrap()
            .unpause()
    }
    fn advance_time_by(&mut self, duration: Duration) {
        self.world_mut()
            .get_resource_mut::<Time<Virtual>>()
            .unwrap()
            .advance_by(duration);
        self.update();
    }
    fn advance_time_to(&mut self, duration: Duration) {
        self.world_mut()
            .get_resource_mut::<Time<Virtual>>()
            .unwrap()
            .advance_to(duration);
        self.update();
    }
}
