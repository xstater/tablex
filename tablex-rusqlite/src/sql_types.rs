use std::num::*;

use rusqlite::ToSql;

/// Sql type Info 
pub trait SqlType : ToSql {
    /// The type name in SQL. "INTEGER", "Text"..
    fn type_name() -> &'static str;

    /// Whether this type is nullable
    fn is_nullable() -> bool;
}

impl<T: SqlType> SqlType for Option<T> {
    fn type_name() -> &'static str {
        T::type_name()
    }

    fn is_nullable() -> bool {
        true
    }
}

macro_rules! impl_sql_type {
    ($sql_type: ty, $type_name: expr) => {
        impl SqlType for $sql_type {
            fn type_name() -> &'static str {
                $type_name
            }

            fn is_nullable() -> bool { false }
        }
    };
}

impl_sql_type!(i8, "INTEGER");
impl_sql_type!(u8, "INTEGER");
impl_sql_type!(i16, "INTEGER");
impl_sql_type!(u16, "INTEGER");
impl_sql_type!(i32, "INTEGER");
impl_sql_type!(u32, "INTEGER");
impl_sql_type!(i64, "INTEGER");
impl_sql_type!(u64, "INTEGER");

impl_sql_type!(NonZeroI8, "INTEGER");
impl_sql_type!(NonZeroU8, "INTEGER");
impl_sql_type!(NonZeroI16, "INTEGER");
impl_sql_type!(NonZeroU16, "INTEGER");
impl_sql_type!(NonZeroI32, "INTEGER");
impl_sql_type!(NonZeroU32, "INTEGER");
impl_sql_type!(NonZeroI64, "INTEGER");
impl_sql_type!(NonZeroU64, "INTEGER");

impl_sql_type!(bool, "INTEGER");

impl_sql_type!(f32, "REAL");
impl_sql_type!(f64, "REAL");

impl_sql_type!(String, "TEXT");

