use crate::TableInfo;


/// The column information
#[derive(Debug)]
pub struct Column<ExtraTableInfo, ExtraColumnInfo>
where 
    ExtraTableInfo: 'static,
    ExtraColumnInfo: 'static,
{
    pub table: &'static TableInfo<ExtraTableInfo, ExtraColumnInfo>,
    /// The name of the column in the database table
    pub column_name: &'static str,
    /// The name of the corresponding field in the struct
    pub field_name: &'static str,
    /// The byte offset of the field within the struct
    pub offset: usize,
    /// The size in bytes of the field
    pub size: usize,
    /// Extra information about the column, can be used for additional metadata
    pub extra: ExtraColumnInfo,
}
