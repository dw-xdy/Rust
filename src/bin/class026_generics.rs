// fn return_number<T>(number: T) -> T {
//     println!("Here is your number.");
//     number
// }
//
// // we can write anything in <>, like: thing, aaaaaa or fdsahuisadf.
// // but we always just use : T, U, K, V.
// fn get_number<dfkslajkljdaf>(number: dfkslajkljdaf) -> dfkslajkljdaf {
//     number
// }
//
// fn main() {
//     let my_number = return_number(String::from("kdshagiouonfkdsahfioaasdhifa"));
//     let my_another_number = get_number(String::from("kfdsahfoisandsafhioaasdfhisa"));
//     println!("{my_number}");
//     println!("{my_another_number}");
// }

// use std::fmt::Debug;
//
// #[derive(Debug)]
// struct Animal {
//     name: String,
//     age: u8,
// }
//
// fn print_item<T: Debug>(item: T) {
//     println!("Here is your item: {:#?}", item);
// }
//
// fn main() {
//     let charlie = Animal {
//         name: "Charlie".to_string(),
//         age: 1,
//     };
//
//     let number = 55;
//
//     print_item(charlie);
//     print_item(number);
// }

use std::cmp::PartialOrd;
use std::fmt::Display;

// where(keyword) make code easy read.
// so we don't need to write too long code
// (like TypeScript's Type gymnastics that's too 😣)
fn compare_and_display<T, U>(statement: T, num_1: U, num_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!(
        "{statement}! Is {num_1} greater than {num_2} ? {}",
        num_1 > num_2
    );
}

fn main() {
    compare_and_display("Listen up!", 9, 8);
}
