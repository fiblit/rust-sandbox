fn main() {
    println!("Hello, world!");

    another_function(5, 'c');

    //let x = (let y = 6);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);
}

fn another_function(x:i32, y:char) {
    println!("Another function with x: {}, y:{}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // cannot put a semicolon here
}
