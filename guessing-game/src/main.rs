use rand::Rng;
use std::io;

fn generate_random_number() -> i8 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=10);
    return random_number;
}

fn main() {
    println!("Let's play a guessing game!");
    println!("Can you guess the number I'm thinking of?");
    println!("It is between 1 and 10. You get three tries to guess.");
    println!("Good luck!");

    let mut tries = 3;
    let random_number = generate_random_number();

    println!("I got the number, what's your guess?");

    while tries > 0 {
        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("You havet to guess dummy!");

        let parsed_user_guess = user_guess.trim().parse().unwrap();

        if random_number == parsed_user_guess {
            println!("Bingo! It was indeed {user_guess}");
            return;
        } else if parsed_user_guess > random_number {
            println!("You guessed higher!");
        } else {
            println!("You guessed lower!");
        }

        tries -= 1;

        println!("You have {tries} left.")
    }

    println!("It was {random_number}, better luck next time!");
}
