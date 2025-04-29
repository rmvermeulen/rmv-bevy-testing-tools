use std::time::Duration;

use bevy_time::{Time, Virtual};

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

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use speculoos::{assert_that, option::OptionAssertions, prelude::BooleanAssertions};

    use super::*;
    use crate::prelude::minimal_test_app;

    #[rstest]
    fn pause(#[from(minimal_test_app)] mut app: TestApp) {
        app.pause();
        assert_that!(app.is_paused()).is_true();
    }

    #[rstest]
    fn unpause(#[from(minimal_test_app)] mut app: TestApp) {
        app.unpause();
        assert_that!(app.is_paused()).is_false();
    }

    #[rstest]
    #[case(0.0)]
    #[case(5.3)]
    #[case(123.3)]
    fn advance_time_to(#[from(minimal_test_app)] mut app: TestApp, #[case] seconds: f32) {
        app.pause();
        app.advance_time_to(Duration::from_secs_f32(seconds));
        assert_that!(app.world().get_resource::<Time>().map(|t| t.elapsed_secs()))
            .is_some()
            .is_equal_to(seconds);
    }

    #[rstest]
    #[case(0.0)]
    #[case(5.3)]
    #[case(123.3)]
    fn advance_time_by(#[from(minimal_test_app)] mut app: TestApp, #[case] seconds: f32) {
        app.pause();
        app.advance_time_by(Duration::from_secs_f32(3.0));
        app.advance_time_by(Duration::from_secs_f32(seconds));
        assert_that!(app.world().get_resource::<Time>().map(|t| t.elapsed_secs()))
            .is_some()
            .is_equal_to(3.0 + seconds);
    }
}
