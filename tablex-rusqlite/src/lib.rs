pub use tablex_rusqlite_derive::*;

/// Sql table meta definitions
pub mod meta;

pub use tablex;

// use tablex::Table;

// mod create_table;
// pub use create_table::*;

// pub trait ConnectionExt {
//     fn create_table<T: Table>(&self) -> TableBuilder<'_, T>;
// }

// impl ConnectionExt for rusqlite::Connection {
//     fn create_table<T: Table>(&self) -> TableBuilder<'_, T> {
//         TableBuilder::new(self)
//     }
// }