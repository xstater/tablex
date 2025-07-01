mod table_info;
pub use table_info::*;

mod column;
pub use column::*;

/// Represents a table
pub trait Table {
    type ExtraTableInfo;
    type ExtraColumnInfo;

    /// Get the information of table
    fn table_info() -> &'static TableInfo<Self::ExtraTableInfo, Self::ExtraColumnInfo>;

    /// Get the reference of column field
    /// # details
    /// - T must be same as the field
    fn value_ref<T: 'static>(&self, column: &Column<Self::ExtraTableInfo, Self::ExtraColumnInfo>) -> Option<&T>;
    /// Get the mutable reference of column field
    /// # details
    /// - T must be same as the field
    fn value_mut<T: 'static>(&mut self, column: &Column<Self::ExtraTableInfo, Self::ExtraColumnInfo>) -> Option<&mut T>;
}


