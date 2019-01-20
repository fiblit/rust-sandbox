fn main() {
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if number {
    //     println!("number was tree");
    // }

    if number != 0 {
        println!("number wasn't zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is div by 3");
    } else if number % 2 == 0 {
        println!("number is div by 2");
    } else {
        println!("number is not div by 4, 3, or 2");
    }

    let number = if number % 4 == 0 {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    // let number =  if true {
    //     5
    // } else {
    //     "six"
    // };
    // println!("The value of number is: {}", number);
}
