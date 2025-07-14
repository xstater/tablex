use std::marker::PhantomData;

use rusqlite::fallible_iterator::FallibleIterator;

use crate::{Builder, Executor, FromRow, SqlTable};

pub struct SelectRowsBuilder<Table> {
    raw_where_clause: Option<String>,
    _marker: std::marker::PhantomData<Table>,
}

impl<Table> SelectRowsBuilder<Table>
where
    Table: SqlTable,
{
    pub(crate) fn new() -> Self {
        SelectRowsBuilder {
            raw_where_clause: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn filter_raw(mut self, raw_where_clause: impl Into<String>) -> Self {
        self.raw_where_clause = Some(raw_where_clause.into());
        self
    }
}

impl<Table> SelectRowsBuilder<Table>
where
    Table: SqlTable,
{
    fn build_sql(&self) -> String {
        let table_info = Table::table_info();
        let table_name = table_info.table_name;

        if let Some(raw_where_clause) = &self.raw_where_clause {
            format!(
                "SELECT * FROM {} WHERE {}",
                table_name, raw_where_clause
            )
        } else {
            format!("SELECT * FROM {}", table_name)
        }
    }
}

impl<Table> Builder<()> for SelectRowsBuilder<Table>
where
    Table: SqlTable + FromRow,
{
    type Executor<'connection> = SelectRowsExecutor<'connection, Table>;

    fn build<'connection>(
        &self,
        connection: &'connection rusqlite::Connection,
        _params: &(),
    ) -> rusqlite::Result<Self::Executor<'connection>> {
        let sql = self.build_sql();

        let stmt = connection.prepare(&sql)?;

        Ok(SelectRowsExecutor {
            stmt,
            _marker: PhantomData::default(),
        })
    }
}

pub struct SelectRowsExecutor<'connection, Table> {
    stmt: rusqlite::Statement<'connection>,
    _marker: PhantomData<Table>,
}

impl<'connection, Table> Executor for SelectRowsExecutor<'connection, Table> 
where 
    Table: FromRow,
{
    type Output = Vec<Table>;

    fn execute(&mut self) -> rusqlite::Result<Self::Output> {
        let rows = self.stmt
            .raw_query();

        let tables = rows
            .map(|row| Table::from_row(row))
            .collect::<Vec<_>>();

        tables
    }

    fn sql(&mut self) -> String {
        self.stmt.expanded_sql().unwrap()
    }
}
