use rusqlite::{BindIndex, Statement, ToSql, types::Null};

pub trait Params {
    type BindIndex: BindIndex;

    fn params(&self) -> impl Iterator<Item = (Self::BindIndex, &dyn ToSql)>;
}

pub(crate) fn bind_params<'a, I, BI>(params: I, stmt: &mut Statement) -> rusqlite::Result<()>
where
    BI: BindIndex,
    I: IntoIterator<Item = (BI, &'a dyn ToSql)>,
{
    for (index, value) in params {
        stmt.raw_bind_parameter(index, value)?;
    }
    Ok(())
}

impl Params for () {
    type BindIndex = usize;

    fn params(&self) -> impl Iterator<Item = (Self::BindIndex, &dyn ToSql)> {
        [(0, &Null as &dyn ToSql); 0].into_iter()
    }
}

impl<T: ToSql> Params for (T,) {
    type BindIndex = usize;

    fn params(&self) -> impl Iterator<Item = (Self::BindIndex, &dyn ToSql)> {
        [(1, &self.0 as &dyn ToSql)].into_iter()
    }
}

impl<A, B> Params for (A, B)
where
    A: ToSql,
    B: ToSql,
{
    type BindIndex = usize;

    fn params(&self) -> impl Iterator<Item = (Self::BindIndex, &dyn ToSql)> {
        [(1, &self.0 as &dyn ToSql), (2, &self.1 as &dyn ToSql)].into_iter()
    }
}

impl<A, B, C> Params for (A, B, C)
where
    A: ToSql,
    B: ToSql,
    C: ToSql,
{
    type BindIndex = usize;

    fn params(&self) -> impl Iterator<Item = (Self::BindIndex, &dyn ToSql)> {
        [
            (1, &self.0 as &dyn ToSql),
            (2, &self.1 as &dyn ToSql),
            (3, &self.2 as &dyn ToSql),
        ]
        .into_iter()
    }
}

impl<A, B, C, D> Params for (A, B, C, D)
where
    A: ToSql,
    B: ToSql,
    C: ToSql,
    D: ToSql,
{
    type BindIndex = usize;

    fn params(&self) -> impl Iterator<Item = (Self::BindIndex, &dyn ToSql)> {
        [
            (1, &self.0 as &dyn ToSql),
            (2, &self.1 as &dyn ToSql),
            (3, &self.2 as &dyn ToSql),
            (4, &self.3 as &dyn ToSql),
        ]
        .into_iter()
    }
}

impl<A, B, C, D, E> Params for (A, B, C, D, E)
where
    A: ToSql,
    B: ToSql,
    C: ToSql,
    D: ToSql,
    E: ToSql,
{
    type BindIndex = usize;

    fn params(&self) -> impl Iterator<Item = (Self::BindIndex, &dyn ToSql)> {
        [
            (1, &self.0 as &dyn ToSql),
            (2, &self.1 as &dyn ToSql),
            (3, &self.2 as &dyn ToSql),
            (4, &self.3 as &dyn ToSql),
            (5, &self.4 as &dyn ToSql),
        ]
        .into_iter()
    }
}
