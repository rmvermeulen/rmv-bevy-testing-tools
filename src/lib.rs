#![feature(coverage_attribute)]

pub mod testing;
pub use testing::*;

pub use insta::{assert_debug_snapshot, assert_snapshot};
pub use rstest::{fixture, rstest};

#[macro_export]
macro_rules! set_snapshot_suffix {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(format!($($expr,)*));
        let _guard = settings.bind_to_scope();
    }
}
