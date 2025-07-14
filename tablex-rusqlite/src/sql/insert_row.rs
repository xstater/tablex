use std::marker::PhantomData;

use crate::{bind_params, Builder, Conflict, Executor, Params, SqlTable};

pub struct InsertRowBuilder<Table> {
    conflict: Option<Conflict>,
    with_auto_increment: bool,
    _marker1: PhantomData<Table>,
}

impl<Table> InsertRowBuilder<Table>
where
    Table: SqlTable,
{
    pub(crate) fn new() -> Self {
        InsertRowBuilder {
            conflict: None,
            with_auto_increment: false,
            _marker1: PhantomData::default(),
        }
    }
}

impl<Table> InsertRowBuilder<Table>
where
    Table: SqlTable,
{
    pub fn or(mut self, conflict: Conflict) -> Self {
        self.conflict = Some(conflict);
        self
    }

    pub fn with_auto_increment(mut self) -> Self {
        self.with_auto_increment = true;
        self
    }
}

impl<T> InsertRowBuilder<T>
where
    T: SqlTable,
{
    fn build_sql(&self) -> String {
        let table_info = T::table_info();
        let table_name = table_info.table_name;

        let or = self
            .conflict
            .as_ref()
            .map(|conflict| format!(" OR {}", conflict))
            .unwrap_or_default();

        let columns = table_info
            .columns
            .iter()
            // 开启自增时，不需要发送自增的列
            .filter(|column| {
                if self.with_auto_increment {
                    !column.extra.is_auto_increment
                } else {
                    true
                }
            })
            .map(|column| column.column_name)
            .collect::<Vec<_>>();

        let placeholders = table_info
            .columns
            .iter()
            // 开启自增时，不需要发送自增的列
            .filter(|column| {
                if self.with_auto_increment {
                    !column.extra.is_auto_increment
                } else {
                    true
                }
            })
            .map(|column| format!(":{}", column.field_name))
            .collect::<Vec<_>>();

        format!(
            "INSERT {} INTO {} ({}) VALUES ({})",
            or,
            table_name,
            columns.join(", "),
            placeholders.join(", ")
        )
    }
}

impl<T> Builder<T> for InsertRowBuilder<T>
where
    T: Params<BindIndex = &'static str> + SqlTable,
{
    type Executor<'connection> = InsertRowExecutor<'connection, T>;

    fn build<'connection>(
        &self,
        connection: &'connection rusqlite::Connection,
        params: &T,
    ) -> rusqlite::Result<Self::Executor<'connection>> {
        let sql = self.build_sql();

        let mut stmt = connection.prepare(&sql)?;

        // 开启自增时，不需要发送自增的列
        if self.with_auto_increment {
            let table_info = T::table_info();
            let auto_increment_columns: Vec<_> = table_info
                .columns
                .iter()
                .filter(|column| column.extra.is_auto_increment)
                .collect();
            let params = params
                .params()
                .filter(|(index, _)| {
                    // 只发送非自增列
                    !auto_increment_columns
                        .iter()
                        // 这里要去掉一个`:`
                        .any(|column| column.field_name == (&index[1..]))
                });
            bind_params(params, &mut stmt)?;
        } else {
            bind_params(params.params(), &mut stmt)?;
        }

        Ok(InsertRowExecutor {
            stmt,
            _marker: PhantomData,
        })
    }
}

pub struct InsertRowExecutor<'conn, T> {
    stmt: rusqlite::Statement<'conn>,
    _marker: PhantomData<T>,
}

impl<'conn, T> Executor for InsertRowExecutor<'conn, T>
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
