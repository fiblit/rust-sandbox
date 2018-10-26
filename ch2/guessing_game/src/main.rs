use std::io;

fn main() {
    println!("");

    println!("Please input your guess.");

    let mut guess = String::new();

    // Take whatever is type and put it in guess.
    io::stdin().read_line(&mut guess)
    // Check that it is `Ok`
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
