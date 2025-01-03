//! bedy server.

// TEMP:
#![allow(missing_docs)]

pub mod extractor {
    pub struct Param<T>(pub T);
}
pub(crate) use extractor::Param;

pub mod index {
    use bedy_common::Id;

    pub trait Index<O> {}

    // Index for Id is special because T is stored together with that index.
    // Other indexes point to the Id index.
    //
    // This makes generated API's resusable for different usecases without too
    // much of a performance loss:
    //
    // ```
    // Little extra overhead lost from dropping city since it's placed adjecently to the Id
    // if let Some((city_id, _)) = db.city().find_by_name(city_name).await {
    //     db.person().people_within(city_id)
    // }
    // ```
    impl<T> Index<T> for Id<T> {}
}
pub(crate) use index::Index;

pub mod query {
    mod read {
        use std::{
            borrow::{Borrow, Cow},
            marker::PhantomData,
        };

        use bedy_common::{Id, Record};

        use crate::*;

        pub struct Query<T, K = Id<T>>(PhantomData<T>, PhantomData<K>);

        impl<T, K> Query<T, K> {
            pub fn iter(&self) -> QueryIter<T> {
                todo!()
            }

            pub fn find(&self, key: impl Borrow<K>) -> Option<Record<T>>
            where
                K: Index<T>,
            {
                todo!()
            }
        }
    }
    pub use read::Query;

    mod write {
        use std::marker::PhantomData;

        use bedy_common::{Id, Record};

        use crate::*;

        pub struct QueryMut<T>(PhantomData<T>);

        impl<T> QueryMut<T> {
            pub fn insert(&mut self, object: &T) -> Id<T> {
                todo!()
            }

            /// Returns the previous object, if any
            pub fn update(&mut self, record: Record<T>) -> Option<T> {
                todo!()
            }

            /// Returns the deleted object, if any
            pub fn delete(&mut self, id: Id<T>) -> Option<T> {
                todo!()
            }
        }
    }
    pub use write::QueryMut;

    mod iter {
        use std::marker::PhantomData;

        use bedy_common::Record;

        pub struct QueryIter<T>(PhantomData<T>);

        impl<T> Iterator for QueryIter<T> {
            type Item = Record<T>;

            fn next(&mut self) -> Option<Self::Item> {
                todo!()
            }
        }
    }
    pub(crate) use iter::QueryIter;
}
pub(crate) use query::*;
