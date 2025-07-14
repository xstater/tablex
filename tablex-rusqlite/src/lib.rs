use std::fmt::Display;

pub use tablex_rusqlite_table_derive::*;
pub use tablex_rusqlite_params_derive::*;
pub use tablex_rusqlite_from_row_derive::*;
/// Sql table meta definitions
pub mod meta;

mod params;
pub use params::*;

mod from_row;
pub use from_row::*;

pub use tablex;

pub mod sql;

mod sql_table;
pub use sql_table::*;

mod sql_types;
pub use sql_types::*;

mod builder;
pub use builder::*;

mod executor;
pub use executor::*;


pub enum Conflict {
    Rollback,
    Abort,
    Fail,
    Ignore,
    Replace,
}

impl Display for Conflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Conflict::Rollback => write!(f, "ROLLBACK"),
            Conflict::Abort => write!(f, "ABORT"),
            Conflict::Fail => write!(f, "FAIL"),
            Conflict::Ignore => write!(f, "IGNORE"),
            Conflict::Replace => write!(f, "REPLACE"),
        }
    }
}