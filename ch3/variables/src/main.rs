fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let spaces = "  ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    // let mut spaces = "   ";
    // spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {}", guess);
}
