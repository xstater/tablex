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
    pub columns: &'static [&'static Column<ExtraColumnInfo>],
    /// Extra information about the table, can be used for additional metadata
    pub extra: ExtraTableInfo,
}

impl<ExtraTableInfo, ExtraColumnInfo> TableInfo<ExtraTableInfo, ExtraColumnInfo>
where
    ExtraTableInfo: 'static,
    ExtraColumnInfo: 'static,
{
    pub fn has_column(&self, column: &Column<ExtraColumnInfo>) -> bool {
        self.columns
            .iter()
            .any(|c| {
                ((*c) as *const _) == (column as *const Column<ExtraColumnInfo>)
            })
    }
}