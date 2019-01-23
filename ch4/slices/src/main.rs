fn main() {
    let s  = String::from("hello world");

    let hello = &s[..5];
    let world = &s[6..=10];
    let hey_world = &s[..];
    println!("{}, {}\n{}", hello, world, hey_world);

//    let mut my_string = String::from("another world");
//    let word = first_word(&s);
//    s.clear();
//    println!("the first word is: {}", word);

    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    let my_literal = "hello world";
    let word = first_word(&my_literal[..]);
    let word = first_word(my_literal); // this works, too

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s:&str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // I suppose this makes sense, it just still seems silly.
    &s[..]
}
