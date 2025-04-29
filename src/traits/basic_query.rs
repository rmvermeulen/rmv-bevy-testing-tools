use bevy_ecs::query::QueryData;

use crate::prelude::TestApp;

pub trait BasicQuery {
    fn query_any<'a, Q, C>(&mut self) -> bool
    where
        Q: QueryData<Item<'a> = C>;
}

impl BasicQuery for TestApp {
    fn query_any<'a, Q, C>(&mut self) -> bool
    where
        Q: QueryData<Item<'a> = C>,
    {
        let mut q = self.world_mut().query::<Q>();
        q.iter(self.world()).next().is_some()
    }
}
