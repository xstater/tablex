use crate::Executor;

pub trait Builder<Params: crate::Params> {
    type Executor<'connection>: Executor;

    fn build<'connection>(
        &self,
        connection: &'connection rusqlite::Connection,
        params: &Params,
    ) -> rusqlite::Result<Self::Executor<'connection>>;
}
