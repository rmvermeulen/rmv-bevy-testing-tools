use bevy_ecs::query::{QueryFilter, QuerySingleError, ReadOnlyQueryData, WorldQuery};

use crate::prelude::TestApp;

pub trait ImmediateQuery {
    fn query_single<D>(&mut self) -> Result<<D as WorldQuery>::Item<'_>, QuerySingleError>
    where
        D: ReadOnlyQueryData;
    fn query_single_filtered<D, F>(
        &mut self,
    ) -> Result<<D as WorldQuery>::Item<'_>, QuerySingleError>
    where
        D: ReadOnlyQueryData,
        F: QueryFilter;
    fn query_collect<D, C>(&mut self) -> C
    where
        D: ReadOnlyQueryData,
        for<'a> C: std::iter::FromIterator<<D as bevy_ecs::query::WorldQuery>::Item<'a>>;
    #[cfg(feature = "iter_tools")]
    fn query_vec<D>(&mut self) -> Vec<<D as WorldQuery>::Item<'_>>
    where
        D: ReadOnlyQueryData;
}

impl ImmediateQuery for TestApp {
    fn query_single<D>(&mut self) -> Result<<D as WorldQuery>::Item<'_>, QuerySingleError>
    where
        D: ReadOnlyQueryData,
    {
        let mut query = self.world_mut().query::<D>();
        query.get_single(self.world_mut())
    }
    fn query_single_filtered<D, F>(
        &mut self,
    ) -> Result<<D as WorldQuery>::Item<'_>, QuerySingleError>
    where
        D: ReadOnlyQueryData,
        F: QueryFilter,
    {
        let mut query = self.world_mut().query_filtered::<D, F>();
        query.get_single(self.world_mut())
    }
    #[cfg(feature = "iter_tools")]
    fn query_vec<D>(&mut self) -> Vec<<D as WorldQuery>::Item<'_>>
    where
        D: ReadOnlyQueryData,
    {
        use iter_tools::Itertools;

        let mut query = self.world_mut().query::<D>();
        query.iter(self.world_mut()).collect_vec()
    }

    fn query_collect<D, C>(&mut self) -> C
    where
        D: ReadOnlyQueryData,
        for<'a> C: std::iter::FromIterator<<D as bevy_ecs::query::WorldQuery>::Item<'a>>,
    {
        let mut query = self.world_mut().query::<D>();
        let result = query.iter(self.world_mut()).collect::<C>();
        result
    }
}
