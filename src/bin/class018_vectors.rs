// fn main() {
//     let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     // Everything is the same as above except we added vec!.
//     let three_to_five = &vec_of_ten[2..5];
//     let start_at_two = &vec_of_ten[1..];
//     let end_at_five = &vec_of_ten[..5];
//     let everything = &vec_of_ten[..];
//
//     println!("Three to five: {:?},
// start at two: {:?}
// end at five: {:?}
// everything: {:?}", three_to_five, start_at_two, end_at_five, everything);
// }
// fn main() {
//     let mut my_vec = Vec::new();
//     my_vec.push("kaishi");
//
//     println!("{:?}", my_vec);
//
//     // different to create Vec .
//     let mut my_vec: Vec<String> = Vec::new();
// }
fn main() {
    let mut num_vec = Vec::new();
    // that also 2 * capacity to increase.
    // let mut num_vec: Vec<char> = Vec::with_capacity(5);

    println!("{}", num_vec.capacity()); // 0 elements: prints 0
    num_vec.push('a'); // add one character
    println!("{}", num_vec.capacity()); // 1 element: prints 4. Vecs with 1 item always start with capacity 4
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // 4 elements: still prints 4.
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // prints 8. We have 5 elements, but it doubled 4 to 8 to make space
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // prints 16. We have 9 elements, but it doubled  8 to 16 to make space

    // must be &str, not: String.
    let my_vec: Vec<&str> = ["kfdsa", "kfdsahlk", "giodsana"].into();
    println!("{:?}", my_vec);

    let my_vec: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [9, 0, 10].into(); // Vec<_> means "choose the Vec type for me"
    // Rust will choose Vec<i32>
    let my_vec2: Vec<_> = ["first", "second", "third"].into(); // Vec<_> means "choose the Vec type for me"
    // Rust will choose Vec<&str>
}
