#[allow(unused_macros)]
macro_rules! trait_mod {
    ($name:ident) => {
        pub mod $name;
        pub use $name::*;
    };
}

#[cfg(feature = "trait_query")]
trait_mod!(basic_query);
#[cfg(feature = "trait_collect_events")]
trait_mod!(collect_events);
#[cfg(feature = "trait_query")]
trait_mod!(immediate_query);
#[cfg(feature = "trait_manage_state")]
trait_mod!(manage_state);
#[cfg(feature = "trait_send_events")]
trait_mod!(send_events);
#[cfg(feature = "trait_time_controls")]
trait_mod!(time_controls);
