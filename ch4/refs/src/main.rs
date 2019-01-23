fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    //change(&s1);

    {
        let r1 = &s1;
        let r2 = &s1;
        // Houston, we now have a problem! r1, and r2 don't expect s to change!
        let r3 = &mut s1;
    }

    {
        // all good
        let r1 = &mut s1;
        println!("{}", r1);
    } // r1 goes out of scope.

    change_mut(&mut s1);
    // NOPE!
    //let r2 = &mut s1;

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but does not drop anything because it has no ownership

// NOPE!
// fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s would be dropped here

fn dangle() -> String {
    let s = String::from("hello");
    s
}
