#![feature(coverage_attribute)]

mod event_collector;
pub use event_collector::*;

#[cfg(any(test, feature = "rstest"))]
mod fixtures;
#[cfg(any(test, feature = "rstest"))]
pub use fixtures::*;

#[cfg(feature = "insta")]
#[macro_export]
macro_rules! set_snapshot_suffix {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(format!($($expr,)*));
        let _guard = settings.bind_to_scope();
    }
}
