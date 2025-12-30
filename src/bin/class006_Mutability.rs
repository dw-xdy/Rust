fn main() {
    // the `mut` keyword make the number is mutable (can change)
    let mut my_number = 8; // This is an i32
    println!("{}", my_number); // prints 8
    my_number = 9;

    // that's can't be, because we need a i32, not a str!
    // can't like the JavaScript programing language
    // my_number = "dhsaiufhdjaskhjkhjk";
    println!("{}", my_number) // Prints 8
}
