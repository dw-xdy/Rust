// fn main() {
//     let my_number = 8; // This is an i32
//     println!("{}", my_number); // prints 8
//     // This is a f64 with the same name.
//     // But it's not the first my_number - it is completely different!
//     // shadowing:
//     let my_number = 9.2_f64;
//
//     println!("{}", my_number) // Prints 9.2
// }
fn times_two(number: i32) -> i32 {
    number * 2
}

fn main() {
    let final_number = {
        // we can use the shadow to make us take variable quickly.
        let y = 10;
        let x = 9; // x starts at 9
        let x = times_two(x); // shadow with new x: 18
        let x = x + y; // shadow with new x: 28
        x // return x: final_number is now the value of x
    };
    println!("The number is now: {}", final_number)
}
