struct User {
    name: String,
    age: u8,
    is_active: bool,
}

struct Users(User, User);

fn main() {
    let user_one: User = get_user_one();

    println!(
        "User One Name: {}\nUser One Age: {}\nUser One IsActive: {}",
        user_one.name, user_one.age, user_one.is_active
    );

    let users = get_users();

    println!(
        "User Two Name: {}\nUser Two Age: {}\nUser Two IsActive: {}",
        users.1.name, users.1.age, users.1.is_active
    )
}

fn get_user_one() -> User {
    let user_one = User {
        name: String::from("John Doe"),
        age: 69,
        is_active: false,
    };

    return user_one;
}

fn get_user_two() -> User {
    let user_two = User {
        name: String::from("Jane Doe"),
        age: 69,
        is_active: true,
    };

    return user_two;
}

fn get_users() -> Users {
    let user_one = get_user_one();
    let user_two = get_user_two();

    let users = Users(user_one, user_two);

    return users;
}
