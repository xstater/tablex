use tablex::Table;

mod create_table;
pub use create_table::*;

pub trait ConnectionExt {
    fn create_table<T: Table>(&self) -> TableBuilder<'_, T>;
}

impl ConnectionExt for rusqlite::Connection {
    fn create_table<T: Table>(&self) -> TableBuilder<'_, T> {
        TableBuilder::new(self)
    }
}