
use crate::prelude::TestApp;

// FIX: no longer compiles with 0.17
#[doc = include_str!("./immediate_query.md")]
pub trait ImmediateQuery {
    // fn query_single<D>(&mut self) -> Result<<D as QueryData>::Item<'_, '_>, QuerySingleError>
    // where
    //     D: ReadOnlyQueryData;

    // fn query_single_filtered<D, F>(
    //     &mut self,
    // ) -> Result<<D as QueryData>::Item<'_, '_>, QuerySingleError>
    // where
    //     D: ReadOnlyQueryData,
    //     F: QueryFilter;

    // fn query_collect<D, C>(&mut self) -> C
    // where
    //     D: ReadOnlyQueryData,
    //     for<'w, 's> C: std::iter::FromIterator<<D as QueryData>::Item<'w, 's>>;

    // #[cfg(feature = "itertools")]
    // fn query_vec<D>(&mut self) -> Vec<<D as QueryData>::Item<'_, '_>>
    // where
    //     D: ReadOnlyQueryData;
}

impl ImmediateQuery for TestApp {
    // fn query_single<D>(&mut self) -> Result<<D as QueryData>::Item<'_, '_>, QuerySingleError>
    // where
    //     D: ReadOnlyQueryData,
    // {
    //     let mut query = self.world_mut().query::<D>();
    //     query.single(self.world_mut())
    // }

    // fn query_single_filtered<D, F>(
    //     &mut self,
    // ) -> Result<<D as QueryData>::Item<'_, '_>, QuerySingleError>
    // where
    //     D: ReadOnlyQueryData,
    //     F: QueryFilter,
    // {
    //     let mut query = self.world_mut().query_filtered::<D, F>();
    //     query.single(self.world_mut())
    // }

    // fn query_collect<D, C>(&mut self) -> C
    // where
    //     D: ReadOnlyQueryData,
    //     for<'a> C: std::iter::FromIterator<<D as bevy_ecs::query::QueryData>::Item<'a, 'a>>,
    // {
    //     let mut query = self.world_mut().query::<D>();
    //     let result = query.iter(self.world_mut()).collect::<C>();
    //     result
    // }

    // #[cfg(feature = "itertools")]
    // fn query_vec<'a, D>(&'a mut self) -> Vec<<D as QueryData>::Item<'a, 'a>>
    // where
    //     D: ReadOnlyQueryData + 'a,
    // {
    //     use itertools::Itertools;
    //
    //     let mut query = self.world_mut().query::<D>();
    //     query.iter(self.world_mut()).collect_vec()
    // }
}
