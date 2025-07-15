use crate::{bind_params, Builder, Executor, FromRow, Params};
use rusqlite::{fallible_iterator::FallibleIterator, Connection};

pub trait ConnectionExt {
    fn execute<B, P>(
        &self,
        builder: &B,
        params: &P,
    ) -> rusqlite::Result<<<B as Builder<P>>::Executor<'_> as Executor>::Output>
    where
        B: Builder<P>,
        P: Params;

    fn query_raw<P, R>(&self, sql: &str, params: &P) -> rusqlite::Result<Vec<R>>
    where
        P: Params,
        R: FromRow;
}

impl ConnectionExt for Connection {
    fn execute<B, P>(
        &self,
        builder: &B,
        params: &P,
    ) -> rusqlite::Result<<<B as Builder<P>>::Executor<'_> as Executor>::Output>
    where
        B: Builder<P>,
        P: Params,
    {
        let mut executor = builder.build(self, params)?;
        executor.execute()
    }
    
    fn query_raw<P, R>(&self, sql: &str, params: &P) -> rusqlite::Result<Vec<R>>
    where
        P: Params,
        R: FromRow {
        let mut stmt = self.prepare(sql)?;

        bind_params(params.params(), &mut stmt)?;

        let rows = stmt.raw_query();

        rows.map(|row| R::from_row(row))
            .collect()
    }
}
