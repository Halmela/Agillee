#[doc(inline)]
pub use std;

//pub mod cli;
pub mod commander;
pub mod data {
    pub mod table;
    pub mod database;
}
pub mod models {
    pub mod structure;
    pub mod object;
    pub mod objects;
    pub mod edge;
    pub mod edges;
    pub mod form;
}

pub mod web {
    pub mod routes;
    pub mod templates;
}
