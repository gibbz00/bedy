//! bedy client.

// TEMP:
#![allow(missing_docs)]
#![allow(async_fn_in_trait)]

use bedy_common::Id;

#[rustfmt::skip]
pub trait ClientCrud<T> {
    async fn insert(&self, person: &T) -> Id<T> { todo!() }
    async fn find(&self, id: Id<T>) -> Option<T> { todo!() }
    async fn update(&self, id: Id<T>, object: &T) -> Option<T> { todo!() }
    async fn remove(&self, id: Id<T>) -> Option<T> { todo!() }
}
