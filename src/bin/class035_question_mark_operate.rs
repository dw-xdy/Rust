// use std::num::ParseIntError;
//
// Here has an interesting question,
// the ? (question mark) just return Err(), not Ok(), so if the value is useful,
// you need use Ok().
// fn parse_str(input: &str) -> Result<i32, ParseIntError> {
//     let parsed_number = input.parse::<i32>()?; // Here is the question mark
//     Ok(parsed_number)
// }
//
// //// simple implement like this.
// // fn parse_str(input: &str) -> Result<i32, ParseIntError> {
// //     Ok(input.parse::<i32>()?) // Here is the question mark
// // }
//
// fn main() {
//     let my_vec = vec!["4334", "fdkjsh", "34", "32784", "fdkhs", "35478", "fdshf"];
//
//     for number in my_vec {
//         let ans = parse_str(number);
//         println!("{:?}", ans);
//     }
// }

use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input
        .parse::<u16>()?
        .to_string()
        .parse::<u32>()?
        .to_string()
        .parse::<i32>()?; // Add a ? each time to check and pass it on
    Ok(parsed_number)
}

fn main() {
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_str(item);
        println!("{:?}", parsed);
    }
}
