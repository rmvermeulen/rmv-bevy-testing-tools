#![feature(coverage_attribute)]

pub mod app;
pub mod events;
#[cfg(any(test, feature = "rstest"))]
pub mod fixtures;
pub mod traits;

pub use app::*;
pub use events::*;
#[cfg(any(test, feature = "rstest"))]
pub use fixtures::*;
pub use traits::*;

#[cfg(feature = "insta")]
#[macro_export]
macro_rules! set_snapshot_suffix {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(format!($($expr,)*));
        let _guard = settings.bind_to_scope();
    }
}
