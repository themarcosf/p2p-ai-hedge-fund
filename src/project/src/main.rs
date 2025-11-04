use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::rng().random_range(1..=10);

    println!("Guess the number!");

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        let _ = io::stdin().read_line(&mut guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
