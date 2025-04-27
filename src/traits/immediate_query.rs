use bevy::ecs::query::{QueryData, QueryFilter, QuerySingleError, ReadOnlyQueryData};

use crate::prelude::TestApp;

pub trait ImmediateQuery {
    fn query_single<D>(&mut self) -> Result<<D as QueryData>::Item<'_>, QuerySingleError>
    where
        D: ReadOnlyQueryData;
    fn query_single_filtered<D, F>(
        &mut self,
    ) -> Result<<D as QueryData>::Item<'_>, QuerySingleError>
    where
        D: ReadOnlyQueryData,
        F: QueryFilter;
    fn query_collect<D, C>(&mut self) -> C
    where
        D: ReadOnlyQueryData,
        for<'a> C: std::iter::FromIterator<<D as QueryData>::Item<'a>>;
    #[cfg(feature = "iter_tools")]
    fn query_vec<D>(&mut self) -> Vec<<D as QueryData>::Item<'_>>
    where
        D: ReadOnlyQueryData;
}

impl ImmediateQuery for TestApp {
    fn query_single<D>(&mut self) -> Result<<D as QueryData>::Item<'_>, QuerySingleError>
    where
        D: ReadOnlyQueryData,
    {
        let mut query = self.world_mut().query::<D>();
        query.single(self.world_mut())
    }
    fn query_single_filtered<D, F>(
        &mut self,
    ) -> Result<<D as QueryData>::Item<'_>, QuerySingleError>
    where
        D: ReadOnlyQueryData,
        F: QueryFilter,
    {
        let mut query = self.world_mut().query_filtered::<D, F>();
        query.single(self.world_mut())
    }
    #[cfg(feature = "iter_tools")]
    fn query_vec<D>(&mut self) -> Vec<<D as QueryData>::Item<'_>>
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
        for<'a> C: std::iter::FromIterator<<D as bevy::ecs::query::QueryData>::Item<'a>>,
    {
        let mut query = self.world_mut().query::<D>();
        let result = query.iter(self.world_mut()).collect::<C>();
        result
    }
}
