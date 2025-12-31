// fn main() {
//     let array1 = ["One", "Two"]; // This one is type [&str; 2]
//     let array2 = ["One", "Two", "Five"]; // But this one is type [&str; 3]. Different type!
//     println!("{:#?}", array1);
//
//     let mut my_array = [0_u32; 640];
//     for i in 0..640 {
//         my_array[i] = i as u32;
//     }
//     println!("{:?}", my_array);
// }
fn main() {
    let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // slice (also: [ , ) )
    let three_to_five = &array_of_ten[2..5];
    let start_at_two = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];

    println!(
        "Three to five: {:?},\n start at two: {:?},\n end at five: {:?},\n everything: {:?}",
        three_to_five, start_at_two, end_at_five, everything
    );
}
