use crate::Column;

/// The Information of Table
#[derive(Debug)]
pub struct TableInfo<ExtraTableInfo, ExtraColumnInfo>
where
    ExtraTableInfo: 'static,
    ExtraColumnInfo: 'static
{
    /// The table name
    pub table_name: &'static str,
    /// the Column definitions
    pub columns: &'static [&'static Column<ExtraTableInfo, ExtraColumnInfo>],
    /// Extra information about the table, can be used for additional metadata
    pub extra: ExtraTableInfo,
}
