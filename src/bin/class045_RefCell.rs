// use std::cell::RefCell;
//
// #[derive(Debug)]
// struct User {
//     id: u32,
//     year_registered: u32,
//     username: String,
//     active: RefCell<bool>,
//     // Many other fields
// }
//
// fn main() {
//     let user_1 = User {
//         id: 1,
//         year_registered: 2020,
//         username: "User 1".to_string(),
//         active: RefCell::new(true),
//     };
//
//     println!("{:?}", user_1.active);
//     // The replace() function
//     user_1.active.replace(false);
//     println!("{:?}", user_1.active);
// }

// #[allow(unused)]
// fn main() {
//     let user_1 = User {
//         id: 1,
//         year_registered: 2020,
//         username: "User 1".to_string(),
//         active: RefCell::new(true),
//     };
//
//     let date = 2020;
//
//     user_1
//         .active
//         .replace_with(|_| if date < 2000 { true } else { false });
//     println!("{:?}", user_1.active);
// }
use std::cell::RefCell;
use std::mem::replace;

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
    // Many other fields
}

fn main() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    // Compiler not get you error, because it will make error in run time.
    // let mut borrow_one = user_1.active.borrow_mut(); // first mutable borrow - okay
    // let mut borrow_two = user_1.active.borrow_mut(); // second mutable borrow - not okay
    let mut reference = user_1.active.borrow_mut(); // first mutable borrow - okay
    *reference = false;

    dbg!(&user_1);
    // we'll look this : borrowed because we have a still alive mutable refence.
    // active: RefCell {
    //     value: <borrowed>,
    // }
    // if we use this: std::mem::drop(param), that drop memory.
    std::mem::drop(reference);

    dbg!(&user_1);
    // active: RefCell {
    //     value: false,
    // }
}
