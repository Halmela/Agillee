#[doc(inline)]
pub use std;

pub mod object;
pub mod objects;
pub mod table;
pub mod database;
//pub mod cli;
pub mod edge;
pub mod commander;
pub mod structure;
pub mod edges;

pub mod web {
    pub mod routes;
    pub mod templates;
}
