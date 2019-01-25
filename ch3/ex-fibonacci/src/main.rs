use std::io;

fn main() {
    let mut n = String::new();
    println!("Input n for nth fibonacci number:");
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");
    let n: u32 = n.trim()
        .parse()
        .expect("Failed to input number");

    println!("{}", fib(n));
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
