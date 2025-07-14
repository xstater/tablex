use std::marker::PhantomData;

use crate::{Builder, Executor, SqlTable};

pub struct DropTableBuilder<T> {
    drop_if_exists: bool,
    _marker: PhantomData<T>,
}

impl<T> DropTableBuilder<T>
where
    T: SqlTable,
{
    pub(crate) fn new() -> Self {
        DropTableBuilder {
            drop_if_exists: false,
            _marker: PhantomData::default(),
        }
    }

    pub fn drop_if_exists(mut self) -> Self {
        self.drop_if_exists = true;
        self
    }
}

impl<T> DropTableBuilder<T>
where
    T: SqlTable,
{
    fn build_sql(&self) -> String {
        let table_name = T::table_info().table_name;
        let if_exists = self
            .drop_if_exists
            .then_some("IF EXISTS")
            .unwrap_or_default();

        format!("DROP TABLE {} {}", if_exists, table_name)
    }
}


impl<T> Builder<()> for DropTableBuilder<T>
where
    T: SqlTable,
{
    type Executor<'connection> = DropTableExecutor<'connection, T>;

    fn build<'connection>(
        &self,
        connection: &'connection rusqlite::Connection,
        _params: &(),
    ) -> rusqlite::Result<Self::Executor<'connection>> {
        let sql = self.build_sql();
        let stmt = connection.prepare(&sql)?;
        Ok(DropTableExecutor {
            stmt,
            _marker: PhantomData,
        })
    }
}


pub struct DropTableExecutor<'conn, T> {
    stmt: rusqlite::Statement<'conn>,
    _marker: PhantomData<T>,
}

impl<'conn, T> Executor for DropTableExecutor<'conn, T>
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
