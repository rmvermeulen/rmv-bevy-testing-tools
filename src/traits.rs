#[allow(unused_macros)]
macro_rules! transparent_module {
    ($name:ident) => {
        pub mod $name;
        pub use $name::*;
    };
}

#[cfg(any(all(test, feature = "rstest"), feature = "trait_collect_messages"))]
transparent_module!(collect_messages);
#[cfg(feature = "trait_manage_state")]
transparent_module!(manage_state);
#[cfg(any(all(test, feature = "rstest"), feature = "trait_write_messages"))]
transparent_module!(write_messages);
