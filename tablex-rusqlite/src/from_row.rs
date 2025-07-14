use rusqlite::types::FromSql;

pub trait FromRow {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self>
    where
        Self: Sized;
}

macro_rules! impl_from_row {
    ($type:ty) => {
        impl FromRow for $type {
            fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
                row.get(0)
            }
        }
    };
}

impl_from_row!(i8);
impl_from_row!(u8);
impl_from_row!(i16);
impl_from_row!(u16);
impl_from_row!(i32);
impl_from_row!(u32);
impl_from_row!(i64);
impl_from_row!(u64);

impl_from_row!(f32);
impl_from_row!(f64);

impl_from_row!(String);
impl_from_row!(bool);

impl<T> FromRow for Option<T>
where
    T: FromRow + FromSql,
{
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        row.get(0)
    }
}

impl<A> FromRow for (A,)
where
    A: FromRow + FromSql,
{
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let a = row.get(0)?;
        Ok((a,))
    }
}

impl<A, B> FromRow for (A, B)
where
    A: FromRow + FromSql,
    B: FromRow + FromSql,
{
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let a = row.get(0)?;
        let b = row.get(1)?;
        Ok((a, b))
    }
}

impl<A, B, C> FromRow for (A, B, C)
where
    A: FromRow + FromSql,
    B: FromRow + FromSql,
    C: FromRow + FromSql,
{
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let a = row.get(0)?;
        let b = row.get(1)?;
        let c = row.get(2)?;
        Ok((a, b, c))
    }
}

impl<A, B, C, D> FromRow for (A, B, C, D)
where
    A: FromRow + FromSql,
    B: FromRow + FromSql,
    C: FromRow + FromSql,
    D: FromRow + FromSql,
{
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let a = row.get(0)?;
        let b = row.get(1)?;
        let c = row.get(2)?;
        let d = row.get(3)?;
        Ok((a, b, c, d))
    }
}

impl<A, B, C, D, E> FromRow for (A, B, C, D, E)
where
    A: FromRow + FromSql,
    B: FromRow + FromSql,
    C: FromRow + FromSql,
    D: FromRow + FromSql,
    E: FromRow + FromSql,
{
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let a = row.get(0)?;
        let b = row.get(1)?;
        let c = row.get(2)?;
        let d = row.get(3)?;
        let e = row.get(4)?;
        Ok((a, b, c, d, e))
    }
}

impl<A, B, C, D, E, F> FromRow for (A, B, C, D, E, F)
where
    A: FromRow + FromSql,
    B: FromRow + FromSql,
    C: FromRow + FromSql,
    D: FromRow + FromSql,
    E: FromRow + FromSql,
    F: FromRow + FromSql,
{
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let a = row.get(0)?;
        let b = row.get(1)?;
        let c = row.get(2)?;
        let d = row.get(3)?;
        let e = row.get(4)?;
        let f = row.get(5)?;
        Ok((a, b, c, d, e, f))
    }
}