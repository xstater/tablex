use tablex_rusqlite::{tablex::Table, Table};

// mod generated;

#[derive(Debug, Table)]
#[table(name = "user_info")]
struct UserInfo {
    #[column]
    id: u32,
    #[column(override_name = "user_name", data_type = "TEXT")]
    name: String,
    #[column]
    age: u32,
}

#[derive(Debug, Table)]
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
        id: 1,
        name: "Alice".to_string(),
        age: 30,
    };


    let age = *user.value_ref::<u32>(age_column).unwrap();

    println!("User's age: {}", age);

    {
        let age = user.value_mut(age_column).unwrap();
        *age = 40_u32;
    }

    println!("User's new age: {}", user.age);

    let reference = Transaction::column_from_id().extra.reference.unwrap();

    assert_eq!(reference as *const _, UserInfo::column_id() as *const _)
}
