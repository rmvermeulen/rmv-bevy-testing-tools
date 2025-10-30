use bevy_app::{App, AppExit};
use bevy_derive::{Deref, DerefMut};
use bevy_utils::default;

// TODO: remove, impl traits on bevy app directly?
#[derive(Debug, Deref, DerefMut)]
pub struct TestApp(pub App);

// NOTE: this is now also handled by feature `bevy/bevy_ci_testing`
impl Drop for TestApp {
    fn drop(&mut self) {
        self.world_mut().write_message(AppExit::Success);
    }
}
