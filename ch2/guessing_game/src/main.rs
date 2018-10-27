extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    // Take whatever is type and put it in guess.
    io::stdin().read_line(&mut guess)
    // Check that it is `Ok`
        .expect("Failed to read line");

    println!("You guessed this: {}", guess);
}
