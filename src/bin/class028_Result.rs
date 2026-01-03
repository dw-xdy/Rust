// enum Result<T, E> {
//     ok(T),
//     Err(E),
// }

// fn get_result(input: i32) -> Result<(), ()> {
//     if input == 5 { Ok(()) } else { Err(()) }
// }
//
// fn main() {
//     let input = 5;
//     if get_result(input).is_ok() {
//         println!("{input}");
//     } else {
//         println!("not that number: {input}")
//     }
// }

// fn check_if_five_number(input: i32) -> Result<String, String> {
//     match input {
//         5 => Ok("This input is 5".to_string()),
//         _ => Err("This input is not 5".to_string()),
//     }
// }
//
// fn main() {
//     for i in 2..7 {
//         if check_if_five_number(i).is_ok() {
//             println!("this number is {i}")
//         } else {
//             println!("this number is not 5, but let me show this number for you {i}")
//         }
//     }
// }

// fn check_if_five(number: i32) -> Result<i32, String> {
//     match number {
//         5 => Ok(number),
//         _ => Err("Sorry, the number wasn't five.".to_string()), // This is our error message
//     }
// }
//
// fn main() {
//     let mut result_vec = Vec::new(); // Create a new vec for the results
//
//     for number in 2..7 {
//         result_vec.push(check_if_five(number)); // push each result into the vec
//     }
//
//     println!("{:#?}", result_vec);
// }

use std::num::ParseIntError;

fn return_number(input: &str) -> Result<i32, ParseIntError> {
    // ::<> : turbo fish (because it looks like a fish )
    // to tell rustc to parse my str to i32.
    input.parse::<i32>()
}

fn main() {
    let my_vec = vec!["5", "3", "3.3", "6sty9", "4387"];

    for number in my_vec {
        match return_number(number) {
            Ok(number) => println!("Got a {number}"),
            Err(e) => println!("Didn't work: {e}"),
        }
    }
}
