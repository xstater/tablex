use std::marker::PhantomData;

use crate::{Builder, Executor, SqlTable};

#[derive(Debug)]
pub struct CreateTableBuilder<T> {
    create_if_not_exists: bool,
    _marker: PhantomData<T>,
}

impl<T> CreateTableBuilder<T>
where
    T: SqlTable,
{
    pub(crate) fn new() -> Self {
        CreateTableBuilder {
            create_if_not_exists: false,
            _marker: PhantomData::default(),
        }
    }

    pub fn create_if_not_exists(mut self) -> Self {
        self.create_if_not_exists = true;
        self
    }

    fn build_sql(&self) -> String {
        let table_info = T::table_info();

        let if_not_exists = self
            .create_if_not_exists
            .then_some("IF NOT EXISTS")
            .unwrap_or_default();

        let columns = table_info
            .columns
            .iter()
            .map(|column| {
                let data_type = column.extra.data_type;

                let primary_key = column
                    .extra
                    .is_primary
                    .then_some("PRIMARY KEY")
                    .unwrap_or_default();

                let auto_increment = column
                    .extra
                    .is_auto_increment
                    .then_some("AUTOINCREMENT")
                    .unwrap_or_default();

                let is_unique = column
                    .extra
                    .is_unique
                    .then_some("UNIQUE")
                    .unwrap_or_default();

                let not_null = column
                    .extra
                    .is_not_null
                    .then_some("NOT NULL")
                    .unwrap_or_default();

                let reference = column
                    .extra
                    .reference
                    .as_ref()
                    .map(|reference| {
                        format!(
                            "REFERENCES {}({})",
                            reference.table.table_name, reference.column.column_name
                        )
                    })
                    .unwrap_or_default();

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

        format!(
            "CREATE TABLE {} {} ({})",
            if_not_exists, table_info.table_name, columns
        )
    }
}

impl<T> Builder<()> for CreateTableBuilder<T>
where
    T: SqlTable,
{
    type Executor<'connection> = CreateTableExecutor<'connection, T>;

    fn build<'connection>(
        &self,
        connection: &'connection  rusqlite::Connection,
        _: &(),
    ) -> rusqlite::Result<Self::Executor<'connection>> {
        let sql = self.build_sql();
        let stmt = connection.prepare(&sql)?;

        Ok(CreateTableExecutor{
            stmt,
            _marker: PhantomData::default(),
        })
    }
}

pub struct CreateTableExecutor<'conn, T> {
    stmt: rusqlite::Statement<'conn>,
    _marker: PhantomData<T>,
}

impl<'conn, T> Executor for CreateTableExecutor<'conn, T>
where
    T: SqlTable,
{
    type Output = ();

    fn execute(&mut self) -> rusqlite::Result<Self::Output> {
        self.stmt.raw_execute()?;
        Ok(())
    }

    fn sql(&mut self) -> String {
        self.stmt.expanded_sql().unwrap()
    }
}
