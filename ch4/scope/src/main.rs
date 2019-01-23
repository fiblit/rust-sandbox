fn main() {
    // s is not valid here
    {
        // s is valid here
        let s = "hello";
        // use s
    }
    // s's scope is now over, and s is not valid

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    // fairly obvious
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // why does this work??
    println!("x = {}, y = {}", x, y);
}
