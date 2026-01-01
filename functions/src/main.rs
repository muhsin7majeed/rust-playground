fn main() {
    let num_one = 60;
    let num_two = 9;

    println!("{} + {} = {}", num_one, num_two, sum(num_one, num_two));
}

fn sum(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}
