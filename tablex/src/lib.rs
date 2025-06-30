pub use tablex_derive::*;

mod table_info;
pub use table_info::*;

mod column;
pub use column::*;

/// Represents a table
pub trait Table {
    /// Get the information of table
    fn table_info() -> &'static TableInfo;

    /// Get the reference of column field
    /// # details
    /// - T must be same as the field
    fn value_ref<T: 'static>(&self, column: &Column) -> Option<&T>;
    /// Get the mutable reference of column field
    /// # details
    /// - T must be same as the field
    fn value_mut<T: 'static>(&mut self, column: &Column) -> Option<&mut T>;
}



#[cfg(test)]
mod test {
    use crate::{Column, Table, TableInfo};

    #[derive(Debug)]
    struct UserInfo {
        id: u32,
        name: String,
        age: u32,
    }

    #[derive(Debug)]
    struct Transaction {
        from_id: u32,
        to_id: u32,
    }

    impl UserInfo {
        pub const fn const_table_info() -> &'static TableInfo {
            static COLUMNS: [&Column; 3] = [
                UserInfo::column_id(),
                UserInfo::column_user_name(),
                UserInfo::column_age(),
            ];

            static TABLE_INFO: TableInfo = TableInfo {
                table_name: "UserInfo",
                columns: &COLUMNS,
            };

            &TABLE_INFO
        }

        pub const fn column_id() -> &'static Column {
            static COLUMN: Column = Column {
                table: UserInfo::const_table_info(),
                column_name: "id",
                field_name: "id",
                offset: std::mem::offset_of!(UserInfo, id),
                size: std::mem::size_of::<u32>(),
                data_type: Some("INTEGER"),
                is_primary: true,
                is_unique: false,
                is_auto_increment: false,
                reference: None,
            };
            &COLUMN
        }

        pub const fn column_user_name() -> &'static Column {
            static COLUMN: Column = Column {
                table: UserInfo::const_table_info(),
                column_name: "user_name",
                field_name: "name",
                offset: std::mem::offset_of!(UserInfo, name),
                size: std::mem::size_of::<String>(),
                data_type: Some("TEXT"),
                is_primary: false,
                is_unique: false,
                is_auto_increment: false,
                reference: None,
            };
            &COLUMN
        }

        pub const fn column_age() -> &'static Column {
            static COLUMN: Column = Column {
                table: UserInfo::const_table_info(),
                column_name: "age",
                field_name: "age",
                offset: std::mem::offset_of!(UserInfo, age),
                size: std::mem::size_of::<u32>(),
                data_type: Some("INTEGER"),
                is_primary: false,
                is_unique: false,
                is_auto_increment: false,
                reference: None,
            };
            &COLUMN
        }
    }

    impl Table for UserInfo {
        fn table_info() -> &'static TableInfo {
            Self::const_table_info()
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

    impl Transaction {
        pub const fn const_table_info() -> &'static TableInfo {
            static COLUMNS: [&Column; 2] =
                [Transaction::column_from_id(), Transaction::column_to_id()];

            static TABLE_INFO: TableInfo = TableInfo {
                table_name: "Transaction",
                columns: &COLUMNS,
            };

            &TABLE_INFO
        }

        pub const fn column_from_id() -> &'static Column {
            static COLUMN: Column = Column {
                table: Transaction::const_table_info(),
                column_name: "from_id",
                field_name: "from_id",
                offset: std::mem::offset_of!(Transaction, from_id),
                size: std::mem::size_of::<u32>(),
                data_type: Some("INTEGER"),
                is_primary: false,
                is_unique: false,
                is_auto_increment: false,
                reference: Some(UserInfo::column_id()),
            };
            &COLUMN
        }

        pub const fn column_to_id() -> &'static Column {
            static COLUMN: Column = Column {
                table: Transaction::const_table_info(),
                column_name: "to_id",
                field_name: "to_id",
                offset: std::mem::offset_of!(Transaction, to_id),
                size: std::mem::size_of::<u32>(),
                data_type: Some("INTEGER"),
                is_primary: false,
                is_unique: false,
                is_auto_increment: false,
                reference: Some(UserInfo::column_id()),
            };
            &COLUMN
        }
    }

    impl Table for Transaction {
        fn table_info() -> &'static TableInfo {
            Self::const_table_info()
        }

        fn value_ref<T: 'static>(&self, column: &Column) -> Option<&T> {
            let _ = column;
            todo!()
        }

        fn value_mut<T: 'static>(&mut self, column: &Column) -> Option<&mut T> {
            let _ = column;
            todo!()
        }
    }

    #[test]
    fn basic() {
        let mut user = UserInfo {
            id: 1,
            name: "Alice".to_string(),
            age: 30,
        };

        let table_info = UserInfo::table_info();
        assert_eq!(table_info.table_name, "UserInfo");
        assert_eq!(table_info.columns.len(), 3);

        let user_name_column = UserInfo::column_user_name();
        let age_column = table_info.get_column_by_column_name("age").unwrap();

        assert_eq!(user.value_ref::<String>(user_name_column).unwrap(), "Alice");
        assert_eq!(user.value_ref::<u32>(age_column).unwrap(), &30);

        {
            let age = user.value_mut::<u32>(age_column).unwrap();
            *age = 40;
        }

        assert_eq!(user.age, 40);

        let from_id_column = Transaction::column_from_id().reference.unwrap();
        assert!(Column::ref_eq(from_id_column, UserInfo::column_id()));
    }
}
