fn count_to(from: i32, to: i32) {
    let mut i = from;

    loop {
        println!("I: {i}");
        i += 1;

        if i > to {
            break;
        }
    }
}

fn print_names(list: [&str; 3]) {
    for item in list.iter() {
        println!("Item: {item}");
    }
}

fn timer(seconds: i32) {
    let mut second = seconds;

    while second >= 0 {
        if second == 0 {
            println!("Times up!");
        } else {
            println!("Remaining second: {second}");
        }

        second -= 1;
    }
}

fn main() {
    println!("Hello, world!");

    let from = 69;
    let to = 72;

    count_to(from, to);

    let list: [&str; 3] = ["Yasuo", "Akali", "Ashe"];

    print_names(list);

    timer(5);
}
