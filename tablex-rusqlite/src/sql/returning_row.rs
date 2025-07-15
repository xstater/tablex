use std::marker::PhantomData;

use crate::{Builder, Executor, FromRow, Params, SqlTable, sql::InsertRowBuilder};

pub struct ReturningRowBuilder<Table> {
    insert_row_builder: InsertRowBuilder<Table>,
    _marker: std::marker::PhantomData<Table>,
}

impl<Table> ReturningRowBuilder<Table>
where
    Table: SqlTable,
{
    pub(crate) fn new(insert_row_builder: InsertRowBuilder<Table>) -> Self {
        ReturningRowBuilder {
            insert_row_builder,
            _marker: std::marker::PhantomData,
        }
    }

    fn build_sql(&self) -> String {
        let insert_sql = self.insert_row_builder.build_sql();

        format!("{} RETURNING *", insert_sql)
    }
}

impl<Table> Builder<Table> for ReturningRowBuilder<Table>
where
    Table: SqlTable + Params<BindIndex = &'static str> + FromRow,
{
    type Executor<'connection> = ReturningRowExecutor<'connection, Table>;

    fn build<'connection>(
        &self,
        connection: &'connection rusqlite::Connection,
        params: &Table,
    ) -> rusqlite::Result<Self::Executor<'connection>> {
        let sql = self.build_sql();

        let mut stmt = connection.prepare(&sql)?;

        self.insert_row_builder
            .bind_insert_params(&mut stmt, params)?;

        Ok(ReturningRowExecutor {
            stmt,
            _marker: PhantomData,
        })
    }
}

pub struct ReturningRowExecutor<'connection, Table> {
    stmt: rusqlite::Statement<'connection>,
    _marker: std::marker::PhantomData<Table>,
}

impl<Table> Executor for ReturningRowExecutor<'_, Table>
where
    Table: SqlTable + FromRow,
{
    type Output = Table;

    fn execute(&mut self) -> rusqlite::Result<Self::Output> {
        let mut rows = self.stmt.raw_query();

        let table = {
            let row = rows.next()?.ok_or(rusqlite::Error::QueryReturnedNoRows)?;
            Table::from_row(row)?
        };

        if let Some(_) = rows.next()? {
            return Err(rusqlite::Error::QueryReturnedMoreThanOneRow);
        }

        Ok(table)
    }

    fn sql(&mut self) -> String {
        self.stmt.expanded_sql().unwrap()
    }
}
