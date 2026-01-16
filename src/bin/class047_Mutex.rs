// // Mutex : mutual exclusion.
//
// use std::sync::Mutex;
//
// fn main() {
//     let my_mutex = Mutex::new(5); // A new Mutex<i32>. We don't need to say mut
//     {
//         // we can use it to auto drop
//         let mut mutex_changer = my_mutex.lock().unwrap(); // mutex_changer is a MutexGuard
//         // It has to be mut because we will change it
//         // Now it has access to the Mutex
//         // Let's print my_mutex to see:
//
//         println!("{:?}", my_mutex); // This prints "Mutex { data: <locked> }"
//         // So we can't access the data with my_mutex now,
//         // only with mutex_changer
//
//         println!("{:?}", mutex_changer); // This prints 5. Let's change it to 6.
//
//         *mutex_changer = 6; // mutex_changer is a MutexGuard<i32> so we use * to change the i32
//
//         println!("{:?}", mutex_changer); // Now it says 6
//     }
//
//     // or we can use std::mem::drop(mutex_changer)
//
//     // if mutex_change variable not be dropped, we can't touch it again. (Deadlock)
//     let mut mutex_changer2 = my_mutex.lock().unwrap();
//     println!("{:?}", my_mutex);
//     println!("{:?}", mutex_changer2);
//     *mutex_changer2 = 213;
//     println!("{:?}", mutex_changer2);
// }

use std::fs::Metadata;
use std::sync::Mutex;

#[derive(Debug)]
struct Book<'a> {
    name: Mutex<&'a str>,
    author: Mutex<&'a str>,
    number_sold: Mutex<u32>,
}

fn main() {
    let my_book = Book {
        name: Mutex::new("The Crystal Cave"),
        author: Mutex::new("Mary Stewart"),
        number_sold: Mutex::new(100_000),
    };

    // *my_book.name.lock().unwrap() = "The Hollow Hills";
    //
    // println!("{:?}", my_book);

    // 在这里加一个大括号, 可以将对应的去除, 然后实现修改.
    {
        let mut mutex_changer1 = my_book.author.lock();
    }
    if let Ok(mut mutex) = my_book.author.try_lock() {
        *mutex = "The Hollow Hills";
    }

    println!("{:?}", my_book);
}
