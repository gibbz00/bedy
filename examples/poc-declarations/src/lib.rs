//! Bedy Proof of Concept declarations crate

mod objects {
    pub mod normalized {
        use bedy_common::object::ForeignId;

        pub struct Person {
            pub name: String,
            pub city: ForeignId<City>,
        }

        pub struct City {
            pub name: String,
        }
    }

    pub mod denormalized {
        pub struct PersonDisplay {
            pub name: String,
            pub city: String,
        }
    }
}

mod queries {
    // TODO: generate these automatically?
    mod auto {
        mod person {
            use bedy_common::{
                object::{Id, Record},
                query::{extractor::Param, Query, QueryMut},
            };

            use crate::objects::normalized::Person;

            fn create_person(Param(person): Param<Person>, mut people: QueryMut<Person>) -> Id<Person> {
                people.insert(&person)
            }

            fn read_person(Param(id): Param<Id<Person>>, people: Query<Person>) -> Option<Person> {
                people.find(id)
            }

            fn update_person(Param(record): Param<Record<Person>>, mut people: QueryMut<Person>) {
                people.update(record);
            }

            fn delete_person(Param(id): Param<Id<Person>>, mut people: QueryMut<Person>) {
                people.delete(id);
            }
        }
    }

    mod custom {
        use bedy_common::query::Query;

        use crate::objects::{
            denormalized::PersonDisplay,
            normalized::{City, Person},
        };

        fn display_people(people: Query<Person>, cities: Query<City>) -> impl Iterator<Item = PersonDisplay> {
            people
                .iter()
                .map(move |person| PersonDisplay { name: person.name, city: cities.through(person.city).name })
        }
    }
}

// TEMP: generated client example
mod client {
    use derive_getters::Getters;
    use futures::Stream;

    use crate::objects::{denormalized::*, normalized::*};

    #[derive(Getters)]
    pub struct BedyClient {}

    impl BedyClient {
        // async because we might need to wait for the people and city write locks to be released
        async fn display_people(&self) -> impl Stream<Item = PersonDisplay> {
            todo!()
        }
    }
}
