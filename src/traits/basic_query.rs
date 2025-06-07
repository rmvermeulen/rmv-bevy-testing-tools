use bevy_ecs::query::QueryData;

use crate::prelude::TestApp;

pub trait BasicQuery {
    fn query_any<Q: QueryData>(&mut self) -> bool;
}

#[doc = include_str!("./basic_query.md")]
impl BasicQuery for TestApp {
    fn query_any<Q: QueryData>(&mut self) -> bool {
        let mut q = self.world_mut().query::<Q>();
        q.iter(self.world()).next().is_some()
    }
}
