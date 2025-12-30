// fn main() {
//     {
//         let my_number = 8; // my_number starts here
//         // my_number ends here!
//     }
//
//     println!("Hello, number {}", my_number); // ⚠️ there is no my_number and
//     // println!() can't find it
// }

fn main() {
    let my_number = {
        let second_number = 8;
        second_number + 9 // No semicolon, so the code block returns 8 + 9.
        // It works just like a function
    };

    println!("My number is: {}", my_number);
}
