fn main() {
    const NAME: &str = "John Doe";
    let mut age: i32 = 69;
    let is_active: bool = false;

    println!("Name: {} \nAge: {} \nIsActive: {}", NAME, age, is_active);

    age = 420;

    println!("Updated Age: {}", age);

    let age: i32 = 69;

    println!("Shadow variable age: {}\n", age);
}
