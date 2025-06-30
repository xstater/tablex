use crate::TableInfo;


/// The column information
#[derive(Debug, Clone, Copy)]
pub struct Column {
    pub table: &'static TableInfo,
    /// The name of the column in the database table
    pub column_name: &'static str,
    /// The name of the corresponding field in the struct
    pub field_name: &'static str,
    /// The byte offset of the field within the struct
    pub offset: usize,
    /// The size in bytes of the field
    pub size: usize,
    /// The data type of the column, e.g., "TEXT", "INTEGER"
    pub data_type: Option<&'static str>,
    /// Whether this column is a primary key
    pub is_primary: bool,
    /// Whether this column is auto-incrementing
    pub is_auto_increment: bool,
    /// Whether this column is unique
    pub is_unique: bool,
    /// Optional reference to another table/column
    pub reference: Option<&'static Column>,
}

impl Column {
    pub fn ref_eq<'a, 'b>(column_a: &'a Column, column_b: &'b Column) -> bool{
        let ptr_a = column_a as *const Column;
        let ptr_b = column_b as *const Column;
        ptr_a == ptr_b
    }
}