//! bedy commons.

// TEMP:
#![allow(missing_docs)]
// TEMP:
#![allow(async_fn_in_trait)]

pub mod object {
    mod id {
        use std::marker::PhantomData;

        pub struct Id<T>(uuid::Uuid, PhantomData<T>);
    }
    pub use id::Id;

    mod foreign_id {
        use std::marker::PhantomData;

        pub struct ForeignId<T>(uuid::Uuid, PhantomData<T>);
    }
    pub use foreign_id::ForeignId;

    mod record {
        use crate::*;

        pub struct Record<T> {
            pub id: Id<T>,
            pub object: T,
        }

        impl<T> From<(Id<T>, T)> for Record<T> {
            fn from(val: (Id<T>, T)) -> Self {
                let (id, object) = val;
                Record { id, object }
            }
        }
    }
    pub use record::Record;
}
pub(crate) use object::*;

pub mod query {
    pub mod extractor {
        pub struct Param<T>(pub T);
    }
    pub(crate) use extractor::Param;

    mod read {
        use std::marker::PhantomData;

        use crate::*;

        pub struct Query<T>(PhantomData<T>);

        impl<T> Query<T> {
            pub fn iter(&self) -> QueryIter<T> {
                todo!()
            }

            pub fn find(&self, id: Id<T>) -> Option<T> {
                todo!()
            }

            // ForeignId must point to an object, hence no Option<T> return value
            pub fn through(&self, id: ForeignId<T>) -> T {
                todo!()
            }
        }
    }
    pub use read::Query;

    mod write {
        use std::marker::PhantomData;

        use crate::*;

        pub struct QueryMut<T>(PhantomData<T>);

        impl<T> QueryMut<T> {
            pub fn insert(&mut self, object: &T) -> Id<T> {
                todo!()
            }

            /// Returns the previous object, if any
            pub fn update(&mut self, record: impl Into<Record<T>>) -> Option<T> {
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

        pub struct QueryIter<T>(PhantomData<T>);

        impl<T> Iterator for QueryIter<T> {
            type Item = T;

            fn next(&mut self) -> Option<Self::Item> {
                todo!()
            }
        }
    }
    pub(crate) use iter::QueryIter;
}
pub(crate) use query::*;
