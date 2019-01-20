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

    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("BOOM!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value of a is: {}", a[index]);
        index = index + 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
