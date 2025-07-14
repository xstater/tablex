use tablex_rusqlite::{
    Builder, Executor, FromRow, Params, Table,
    sql::{self},
    tablex::Table,
};

// mod generated;

#[derive(Debug, Table, FromRow, Params)]
#[table(name = "user_info")]
struct UserInfo {
    #[column(primary, auto_increment)]
    id: u32,
    #[column(name = "user_name")]
    name: String,
    #[column]
    age: u32,
    #[column]
    address: Option<String>,
}

#[derive(Debug, Table)]
#[table(name = "tx")]
struct Transaction {
    #[column(reference_table = UserInfo, reference_key = id)]
    from_id: u32,
    #[column(reference_table = UserInfo, reference_key = id)]
    to_id: u32,
    #[column]
    amount: f64,
}

fn main() {
    let age_column = UserInfo::column_age();

    let mut user = UserInfo {
        id: 10,
        name: "Alice".to_string(),
        age: 30,
        address: None,
    };

    let age = *user.value_ref::<u32>(age_column).unwrap();

    println!("User's age: {}", age);

    {
        let age = user.value_mut(age_column).unwrap();
        *age = 40_u32;
    }

    println!("User's new age: {}", user.age);

    let reference = Transaction::column_from_id()
        .extra
        .reference
        .as_ref()
        .unwrap()
        .column;

    assert_eq!(reference as *const _, UserInfo::column_id() as *const _);

    let connection = rusqlite::Connection::open_in_memory().unwrap();
    let mut create_user_info = sql::create_table::<UserInfo>()
        .create_if_not_exists()
        .build(&connection, &())
        .unwrap();
    println!("Create table SQL: {}", create_user_info.sql());
    create_user_info.execute().unwrap();

    let insert_user_info = sql::insert_row().with_auto_increment();
    let mut insert_user_info_1 = insert_user_info.build(&connection, &user).unwrap();
    println!("Insert row SQL 1: {}", insert_user_info_1.sql());
    insert_user_info_1.execute().unwrap();

    let mut insert_user_info_2 = insert_user_info
        .build(
            &connection,
            &UserInfo {
                id: 0,
                name: "Bob".to_string(),
                age: 59,
                address: Some("sb".to_string()),
            },
        )
        .unwrap();
    println!("Insert row SQL 2: {}", insert_user_info_2.sql());
    insert_user_info_2.execute().unwrap();

    let mut select_rows = sql::select_rows::<UserInfo>()
        .filter_raw("age > 40")
        .build(&connection, &())
        .unwrap();
    println!("Select rows SQL: {}", select_rows.sql());
    let user_infos = select_rows.execute().unwrap();
    println!("Selected user infos: {:?}", user_infos);
}
