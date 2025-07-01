use tablex::{Column, TableInfo};


pub struct SqlExtraColumnInfo {
    /// The data type of the column, e.g., "TEXT", "INTEGER"
    pub data_type: Option<&'static str>,
    /// Whether this column is a primary key
    pub is_primary: bool,
    /// Whether this column is auto-incrementing
    pub is_auto_increment: bool,
    /// Whether this column is unique
    pub is_unique: bool,
    /// Optional reference to another table/column
    pub reference: Option<&'static Column<(), Self>>,
}

pub type SqlTableInfo = TableInfo<(), SqlExtraColumnInfo>;
pub type SqlColumnInfo = Column<(), SqlExtraColumnInfo>;

#[cfg(test)]
mod test {
    use tablex::{Column, Table, TableInfo};

    use crate::meta::{SqlColumnInfo, SqlExtraColumnInfo, SqlTableInfo};

    

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
        pub const fn const_table_info() -> &'static SqlTableInfo {
            static COLUMNS: [&SqlColumnInfo; 3] = [
                UserInfo::column_id(),
                UserInfo::column_user_name(),
                UserInfo::column_age(),
            ];

            static TABLE_INFO: SqlTableInfo = SqlTableInfo {
                table_name: "UserInfo",
                columns: &COLUMNS,
                extra: (),
            };

            &TABLE_INFO
        }

        pub const fn column_id() -> &'static SqlColumnInfo {
            static COLUMN: SqlColumnInfo = SqlColumnInfo {
                table: UserInfo::const_table_info(),
                column_name: "id",
                field_name: "id",
                offset: std::mem::offset_of!(UserInfo, id),
                size: std::mem::size_of::<u32>(),
                extra: SqlExtraColumnInfo {
                    data_type: "INTEGER",
                    is_primary: true,
                    is_unique: false,
                    is_auto_increment: false,
                    reference: None,
                },
            };
            &COLUMN
        }

        pub const fn column_user_name() -> &'static SqlColumnInfo {
            static COLUMN: SqlColumnInfo = SqlColumnInfo {
                table: UserInfo::const_table_info(),
                column_name: "user_name",
                field_name: "name",
                offset: std::mem::offset_of!(UserInfo, name),
                size: std::mem::size_of::<String>(),
                extra: SqlExtraColumnInfo {
                    data_type: "TEXT",
                    is_primary: false,
                    is_unique: false,
                    is_auto_increment: false,
                    reference: None,
                },
            };
            &COLUMN
        }

        pub const fn column_age() -> &'static SqlColumnInfo {
            static COLUMN: SqlColumnInfo = SqlColumnInfo {
                table: UserInfo::const_table_info(),
                column_name: "age",
                field_name: "age",
                offset: std::mem::offset_of!(UserInfo, age),
                size: std::mem::size_of::<u32>(),
                extra: SqlExtraColumnInfo {
                    data_type: "INTEGER",
                    is_primary: false,
                    is_unique: false,
                    is_auto_increment: false,
                    reference: None,
                },
            };
            &COLUMN
        }
    }

    impl Table for UserInfo {
        type ExtraTableInfo = ();

        type ExtraColumnInfo = SqlExtraColumnInfo;

        fn table_info() -> &'static TableInfo<Self::ExtraTableInfo, Self::ExtraColumnInfo> {
            Self::const_table_info()
        }

        fn value_ref<T: 'static>(
            &self,
            column: &Column<Self::ExtraTableInfo, Self::ExtraColumnInfo>,
        ) -> Option<&T> {
            let type_id_t = std::any::TypeId::of::<T>();
            if column.field_name == "name" && type_id_t == std::any::TypeId::of::<String>() {
                return Some(unsafe { &*(&self.name as *const _ as *const T) });
            } else if column.field_name == "age" && type_id_t == std::any::TypeId::of::<u32>() {
                return Some(unsafe { &*(&self.age as *const _ as *const T) });
            }
            None
        }

        fn value_mut<T: 'static>(
            &mut self,
            column: &Column<Self::ExtraTableInfo, Self::ExtraColumnInfo>,
        ) -> Option<&mut T> {
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
        pub const fn const_table_info() -> &'static SqlTableInfo {
            static COLUMNS: [&SqlColumnInfo; 2] =
                [Transaction::column_from_id(), Transaction::column_to_id()];

            static TABLE_INFO: SqlTableInfo = SqlTableInfo {
                table_name: "Transaction",
                columns: &COLUMNS,
                extra: (),
            };

            &TABLE_INFO
        }

        pub const fn column_from_id() -> &'static SqlColumnInfo {
            static COLUMN: SqlColumnInfo = SqlColumnInfo {
                table: Transaction::const_table_info(),
                column_name: "from_id",
                field_name: "from_id",
                offset: std::mem::offset_of!(Transaction, from_id),
                size: std::mem::size_of::<u32>(),
                extra: SqlExtraColumnInfo {
                    data_type: "INTEGER",
                    is_primary: false,
                    is_unique: false,
                    is_auto_increment: false,
                    reference: Some(UserInfo::column_id()),
                },
            };
            &COLUMN
        }

        pub const fn column_to_id() -> &'static SqlColumnInfo {
            static COLUMN: SqlColumnInfo = SqlColumnInfo {
                table: Transaction::const_table_info(),
                column_name: "to_id",
                field_name: "to_id",
                offset: std::mem::offset_of!(Transaction, to_id),
                size: std::mem::size_of::<u32>(),
                extra: SqlExtraColumnInfo {
                    data_type: "INTEGER",
                    is_primary: false,
                    is_unique: false,
                    is_auto_increment: false,
                    reference: Some(UserInfo::column_id()),
                },
            };
            &COLUMN
        }
    }

    impl Table for Transaction {
        type ExtraTableInfo = ();
        type ExtraColumnInfo = SqlExtraColumnInfo;

        fn table_info() -> &'static TableInfo<Self::ExtraTableInfo, Self::ExtraColumnInfo> {
            Self::const_table_info()
        }

        fn value_ref<T: 'static>(&self, column: &Column<Self::ExtraTableInfo, Self::ExtraColumnInfo>) -> Option<&T> {
            let _ = column;
            todo!()
        }

        fn value_mut<T: 'static>(&mut self, column: &Column<Self::ExtraTableInfo, Self::ExtraColumnInfo>) -> Option<&mut T> {
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
        let age_column = UserInfo::column_age();

        assert_eq!(user.value_ref::<String>(user_name_column).unwrap(), "Alice");
        assert_eq!(user.value_ref::<u32>(age_column).unwrap(), &30);

        {
            let age = user.value_mut::<u32>(age_column).unwrap();
            *age = 40;
        }

        assert_eq!(user.age, 40);

        let from_id_column = Transaction::column_from_id().extra.reference.unwrap();
        assert_eq!(from_id_column as *const _, UserInfo::column_id() as *const _);
    }
}
