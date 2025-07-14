use tablex::Table;

use crate::meta::SqlExtraColumnInfo;



pub trait SqlTable:
    Sized + Table<ExtraTableInfo = (), ExtraColumnInfo = SqlExtraColumnInfo>
{
}

impl<T> SqlTable for T where T: Table<ExtraTableInfo = (), ExtraColumnInfo = SqlExtraColumnInfo> {}
