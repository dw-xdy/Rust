// // ⚠️️️
// fn take_fifth(value: Vec<i32>) -> i32 {
//     value[4]
// }
//
// fn main() {
//     let new_vec = vec![1, 2];
//     let index = take_fifth(new_vec);
// }

//// Option means maybe it's something or maybe None.
// fn take_fifth(value: Vec<i32>) -> Option<i32> {
//     if value.len() < 5 {
//         // .len() gives the length of the vec.
//         // It must be at least 5.
//         None // maybe None.
//     } else {
//         Some(value[4]) // Maybe something
//     }
// }
//
// fn main() {
//     let new_vec = vec![1, 2];
//     let bigger_vec = vec![1, 2, 3, 4, 5];
//     println!(
//         "{:?}, {:?}",
//         take_fifth(new_vec),  // we can't use .unwrap() to None. (that's an error)
//         take_fifth(bigger_vec).unwrap()
//     );
// }

// fn take_fifth(value: Vec<i32>) -> Option<i32> {
//     if value.len() < 5 {
//         None
//     } else {
//         Some(value[4])
//     }
// }
//
// fn handle_option(my_option: Vec<Option<i32>>) {
//     for item in my_option {
//         match item {
//             Some(number) => println!("Found a {number}!"),
//             None => println!("Found a None!"),
//         }
//     }
// }
//
// fn main() {
//     let new_vec = vec![1, 2];
//     let bigger_vec = vec![1, 2, 3, 4, 5];
//     let mut option_vec = Vec::new(); // Make a new vec to hold our options
//     // The vec is type: Vec<Option<i32>>. That means a vec of Option<i32>.
//
//     option_vec.push(take_fifth(new_vec)); // This pushes "None" into the vec
//     option_vec.push(take_fifth(bigger_vec)); // This pushes "Some(5)" into the vec
//
//     handle_option(option_vec); // handle_option looks at every option in the vec.
//     // It prints the value if it is Some. It doesn't touch it if it is None.
// }

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

// fn main() {
//     let new_vec = vec![1, 2];
//     let bigger_vec = vec![1, 2, 3, 4, 5];
//     let vec_of_vecs = vec![new_vec, bigger_vec];
//     for vec in vec_of_vecs {
//         let inside_number = take_fifth(vec);
//         if inside_number.is_some() {
//             // .is_some() returns true if we get Some, false if we get None
//             println!("We got: {}", inside_number.unwrap()); // now it is safe to use .unwrap() because we already checked
//         } else {
//             println!("We got nothing.");
//         }
//     }
// }

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let vec_of_vecs = vec![new_vec, bigger_vec];
    for vec in vec_of_vecs {
        let inside_number = take_fifth(vec);
        // we have much method to keep safe and simple like this method: unwrap_or(i32).
        println!("The number is : {}", inside_number.unwrap_or(-1));
    }
}
