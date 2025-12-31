// fn main() {
//     let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
//     println!(
//         "Inside the tuple is: \nFirst item: {:?}
// Second item: {:?}
// Third item: {:?}
// Fourth item: {:?}
// Fifth item: {:?}
// Sixth item: {:?}",
//         random_tuple.0,
//         random_tuple.1,
//         random_tuple.2,
//         random_tuple.3,
//         random_tuple.4,
//         random_tuple.5,
//     )
// }

fn main() {
    let str_vec = vec!["one", "two", "three"];

    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]); // call them a, b, and c
    println!("{:?}", b);
}
