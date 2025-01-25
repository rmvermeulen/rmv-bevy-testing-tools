#![feature(coverage_attribute)]

pub mod app;
#[cfg(feature = "speculoos")]
pub mod assertions;
pub mod events;
#[cfg(any(test, feature = "rstest"))]
pub mod fixtures;
pub mod traits;

#[cfg(feature = "insta")]
#[macro_export]
macro_rules! set_snapshot_suffix {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(format!($($expr,)*));
        let _guard = settings.bind_to_scope();
    }
}

pub mod prelude {
    #[cfg(feature = "rstest")]
    pub use super::fixtures::*;
    #[cfg(feature = "insta")]
    pub use super::set_snapshot_suffix;
    pub use super::{app::*, events::*, traits::*};
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "rstest")]
    use rstest::rstest;

    use super::prelude::*;

    #[cfg(feature = "rstest")]
    #[rstest]
    fn can_access_everything(mut test_app: TestApp) {
        #[derive(bevy_ecs::event::Event, Default, Debug, Copy, Clone)]
        struct MyEvent;
        test_app
            .collect_events::<MyEvent>()
            .send_event_default::<MyEvent>();
    }
}
