pub use tablex_derive::*;

/// Represents a table
pub trait Table {
    /// Get the name of the table
    fn name() -> &'static str;
    /// Get the columns of the table
    fn columns() -> &'static [Column];
    /// Get the reference of column field
    /// # details
    /// - T must be same as the field
    fn value_ref<T: 'static>(&self, column: &Column) -> Option<&T>;
    /// Get the mutable reference of column field
    /// # details
    /// - T must be same as the field
    fn value_mut<T: 'static>(&mut self, column: &Column) -> Option<&mut T>;

    /// Get a column by its name
    fn get_column(column_name: &str) -> Option<&'static Column> {
        Self::columns()
            .iter()
            .find(|col| col.column_name == column_name)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Column {
    /// The name of the column
    pub column_name: &'static str,
    pub field_name: &'static str,
    pub offset: usize,
    pub size: usize,
    pub data_type: Option<&'static str>,
    pub is_primary_key: bool,
}

#[cfg(test)]
mod test {
    use crate::{Column, Table};

    #[derive(Debug)]
    struct UserInfo {
        name: String,
        age: u32,
    }

    impl Table for UserInfo {
        fn name() -> &'static str {
            "UserInfo"
        }

        fn columns() -> &'static [Column] {
            static COLUMNS: [Column; 2] = [
                Column {
                    column_name: "user_name",
                    field_name: "name",
                    offset: std::mem::offset_of!(UserInfo, name),
                    size: std::mem::size_of::<String>(),
                    data_type: Some("TEXT"),
                    is_primary_key: true
                },
                Column {
                    column_name: "age",
                    field_name: "age",
                    offset: std::mem::offset_of!(UserInfo, age),
                    size: std::mem::size_of::<u32>(),
                    data_type: Some("INTEGER"),
                    is_primary_key: true
                },
            ];
            &COLUMNS
        }

        fn value_ref<T: 'static>(&self, column: &Column) -> Option<&T> {
            let type_id_t = std::any::TypeId::of::<T>();
            if column.field_name == "name" && type_id_t == std::any::TypeId::of::<String>() {
                return Some(unsafe { &*(&self.name as *const _ as *const T) });
            } else if column.field_name == "age" && type_id_t == std::any::TypeId::of::<u32>() {
                return Some(unsafe { &*(&self.age as *const _ as *const T) });
            }
            None
        }

        fn value_mut<T: 'static>(&mut self, column: &Column) -> Option<&mut T> {
            let type_id_t = std::any::TypeId::of::<T>();
            if column.field_name == "name" && type_id_t == std::any::TypeId::of::<String>() {
                return Some(unsafe { &mut *(&mut self.name as *mut _ as *mut T) });
            } else if column.field_name == "age" && type_id_t == std::any::TypeId::of::<u32>() {
                return Some(unsafe { &mut *(&mut self.age as *mut _ as *mut T) });
            }
            None
        }
    }

    #[test]
    fn basic() {
        let mut user = UserInfo {
            name: "Alice".to_string(),
            age: 30,
        };

        assert_eq!(UserInfo::name(), "UserInfo");
        assert_eq!(UserInfo::columns().len(), 2);

        let user_name_column = UserInfo::get_column("user_name").unwrap();
        let age_column = UserInfo::get_column("age").unwrap();

        assert_eq!(user.value_ref::<String>(user_name_column).unwrap(), "Alice");
        assert_eq!(user.value_ref::<u32>(age_column).unwrap(), &30);

        {
            let age = user.value_mut::<u32>(age_column).unwrap();
            *age = 40;
        }

        assert_eq!(user.age, 40);
    }
}
