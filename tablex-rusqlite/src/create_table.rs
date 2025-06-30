use std::marker::PhantomData;

use tablex::Table;

pub struct TableBuilder<'db, T> {
    connection: &'db rusqlite::Connection,
    _marker: PhantomData<T>,
    create_if_not_exists: bool,
}

impl<'db, T: Table> TableBuilder<'db, T> {
    pub(super) fn new(connection: &'db rusqlite::Connection) -> Self {
        TableBuilder {
            connection,
            _marker: PhantomData,
            create_if_not_exists: false,
        }
    }
}

impl<'db, T: Table> TableBuilder<'db, T> {
    pub fn create_if_not_exists(mut self) -> Self {
        self.create_if_not_exists = true;
        self
    }

    pub fn execute(self) -> rusqlite::Result<()> {
        let table_info = T::table_info();

        let if_not_exists = self
            .create_if_not_exists
            .then_some("IF NOT EXISTS")
            .unwrap_or_default();

        let columns = table_info
            .columns
            .iter()
            .map(|column| {
                // todo: data type 
                let data_type = "";

                let primary_key = column
                    .is_primary
                    .then_some("PRIMARY KEY")
                    .unwrap_or_default();

                let auto_increment = column
                    .is_auto_increment
                    .then_some("AUTOINCREMENT")
                    .unwrap_or_default();

                let is_unique = column
                    .is_unique
                    .then_some("UNIQUE")
                    .unwrap_or_default();

                // todo: option support
                let not_null = "NOT NULL";

                // todo: handle references
                let reference = "";

                format!(
                    "{} {} {} {} {} {} {}",
                    column.column_name,
                    data_type,
                    primary_key,
                    auto_increment,
                    is_unique,
                    not_null,
                    reference
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "CREATE TABLE {} {} ({})",
            if_not_exists,
            table_info.table_name,
            columns
        );

        self.connection.execute(&sql, ()).map(|_| ())
    }
}
