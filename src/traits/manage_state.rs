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

#[cfg(test)]
mod tests {
    use std::env;

    use bevy_state::{
        app::{AppExtStates, StatesPlugin},
        state::{NextState, States},
    };
    use rstest::{fixture, rstest};
    use speculoos::{assert_that, asserting, option::OptionAssertions, string::StrAssertions};

    use crate::{
        prelude::{minimal_test_app, TestApp},
        traits::ManageState,
    };

    #[rstest]
    fn trait_manage_state(#[from(minimal_test_app)] mut app: TestApp) {
        // we don't want to run this test, but it has to compile.
        // this is just so the compiler doesn't remove this code
        if env::var("_IF_IT_COMPILES_ITS_FINE_")
            .map(|s| s == "true")
            .unwrap_or(true)
        {
            return;
        }

        #[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
        enum MyState {
            #[default]
            A,
            B,
        }

        app.init_state::<MyState>();
        app.set_next_state(MyState::B);
    }

    #[derive(States, Debug, Copy, Clone, PartialEq, Eq, Hash)]
    enum MyState {
        First,
        Second,
    }

    #[fixture]
    fn states_app(
        #[from(minimal_test_app)]
        #[with(StatesPlugin)]
        app: TestApp,
    ) -> TestApp {
        app
    }

    #[rstest]
    fn test_app_get_state(#[from(states_app)] mut app: TestApp) {
        asserting!("TestApp::get_state() before MyState exists")
            .that(&app.get_state::<MyState>())
            .is_none();

        app.insert_state(MyState::First);

        asserting!("TestApp::get_state() when MyState exists")
            .that(&app.get_state::<MyState>())
            .is_some()
            .is_equal_to(&MyState::First);
    }

    #[rstest]
    fn test_app_get_next_state(#[from(states_app)] mut app: TestApp) {
        asserting!("TestApp::get_next_state() before MyState exists")
            .that(&app.get_next_state::<MyState>())
            .is_none();

        app.insert_state(MyState::First);

        let next_state = app.get_next_state::<MyState>();

        asserting!("TestApp::get_next_state() when MyState exists")
            .that(&next_state)
            .is_some();
        assert_that!(format!("{:?}", next_state.unwrap()))
            .is_equal_to(format!("{:?}", NextState::<MyState>::Unchanged));
    }

    #[rstest]
    fn test_app_set_next_state(#[from(states_app)] mut app: TestApp) {
        asserting!("TestApp::set_next_state() before MyState exists")
            .that(&app.set_next_state(MyState::First))
            .is_none();

        app.insert_state(MyState::First);
        asserting!("TestApp::set_next_state() before MyState exists")
            .that(&app.set_next_state(MyState::Second))
            .is_some();

        let next_state = app.get_next_state::<MyState>();
        asserting!("TestApp::get_next_state() after set_next_state()")
            .that(&next_state)
            .is_some();
        assert_that!(format!("{:?}", next_state.unwrap()))
            .contains(format!("{:?}", MyState::Second));
    }
}
