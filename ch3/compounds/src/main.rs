fn main() {
    let tup: (i32, f64, u8) = (500, 6.1, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("Y is: {}, 0th tuplet is: {}", y, tup.0);
    let five_hundo = tup.0;
    let six_four = tup.1;

    let a: [i32; 5] = [z, 2, 3, 4, 5];

    let a: [i32; 5] = a;
    println!("3th a: {}", a[3]);

    //let outside = 10;
    //let element = a[outside];
    let inside = 3;
    let element = a[inside];
    println!("This will panic before printing: {}", element);

    
}
