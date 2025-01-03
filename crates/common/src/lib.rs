//! bedy commons.

// TEMP:
#![allow(missing_docs)]

mod id {
    use std::marker::PhantomData;

    #[impl_tools::autoimpl(PartialEq)]
    pub struct Id<T>(uuid::Uuid, PhantomData<T>);
}
pub use id::Id;

pub type Record<T> = (Id<T>, T);
