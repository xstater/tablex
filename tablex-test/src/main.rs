use tablex::Table;

#[derive(Debug, Table)]
#[table(name = "user_info")]
struct UserInfo {
    #[column(override_name = "user_name", data_type = "TEXT")]
    name: String,
    #[column]
    age: u32,
}

fn main() {
    println!("Table name: {}", UserInfo::name());
    println!("Columns: {:?}", UserInfo::columns());

    let age_column = UserInfo::get_column("age").unwrap();

    let mut user = UserInfo {
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
}
