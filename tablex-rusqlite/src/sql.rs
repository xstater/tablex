mod create_table;

pub use create_table::*;

mod drop_table;
pub use drop_table::*;

mod insert_row;
pub use insert_row::*;

mod select_rows;
pub use select_rows::*;

mod returning_row;
pub use returning_row::*;

use crate::SqlTable;

pub fn create_table<T>() -> CreateTableBuilder<T>
where
    T: SqlTable,
{
    CreateTableBuilder::new()
}

pub fn drop_table<T>() -> DropTableBuilder<T>
where
    T: SqlTable,
{
    DropTableBuilder::new()
}

pub fn insert_row<T>() -> InsertRowBuilder<T>
where
    T: SqlTable,
{
    InsertRowBuilder::new()
}


pub fn select_rows<T>() -> SelectRowsBuilder<T>
where
    T: SqlTable,
{
    SelectRowsBuilder::new()
}