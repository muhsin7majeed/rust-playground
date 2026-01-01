fn print_divider() {
    const DIVIDER: &str = "-------";

    println!("{}", DIVIDER);
    return;
}

fn main() {
    let rating: f32 = 3.7;

    let sum = rating + 1.3; // sum is f32

    println!("Sum: {}", sum);

    let emoji: char = '👻';

    println!("Boo {}!", emoji);

    print_divider();

    let penguins_of_madagascar: [char; 4] = ['🐧', '🐧', '🐧', '🐧'];
    let first_penguin = penguins_of_madagascar[0];
    let last_penguin = penguins_of_madagascar[3];

    let number_of_penguind = penguins_of_madagascar.len();

    println!(
        "Total Penguins: {}\nFirst Penguin: {} \nLast Penguin: {}",
        number_of_penguind, first_penguin, last_penguin
    );

    print_divider();

    let admin_tuple: (&str, i32, bool) = ("John Doe", 69, true);

    println!(
        "Admin User;\nName: {}\nAge: {}\nIsActive: {}",
        admin_tuple.0, admin_tuple.1, admin_tuple.2
    );
}
