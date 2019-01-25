use std::io;

fn main() {
    loop {
        println!("Convert from (F)ahrenheit or from (C)elsius?");
        let mut from_temp = String::new();
        io::stdin().read_line(&mut from_temp)
            .expect("Failed to read line.");
        let from_temp = from_temp.trim().to_lowercase();

        println!("Temperature to convert:");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp)
            .expect("Failed to read line.");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if from_temp == "f" {
            println!("{} degrees C", f2c(temp));
            break;
        } else if from_temp == "c" {
            println!("{} degrees F", c2f(temp));
            break;
        }
    }
}

fn f2c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn c2f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
