use crate::Column;

/// The Information of Table
#[derive(Debug, Clone, Copy)]
pub struct TableInfo {
    /// The table name
    pub table_name: &'static str,
    /// the Column definitions
    pub columns: &'static [&'static Column],
}

impl TableInfo {
    pub fn get_column_by_column_name(&self, column_name: &'static str) -> Option<&'static Column> {
        self.columns
            .iter()
            .find(|column| column.column_name == column_name)
            .map(|v| &**v)
    }

    pub fn get_column_by_field_name(&self, field_name: &'static str) -> Option<&'static Column> {
        self.columns
            .iter()
            .find(|column| column.field_name == field_name)
            .map(|v| &**v)
    }

    pub fn ref_eq<'a, 'b>(table_info_a: &'a TableInfo, table_info_b: &'b TableInfo) -> bool{
        let ptr_a = table_info_a as *const TableInfo;
        let ptr_b = table_info_b as *const TableInfo;
        ptr_a == ptr_b
    }
}