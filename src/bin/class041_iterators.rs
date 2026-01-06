// fn main() {
//     let vector1 = vec![1, 2, 3]; // we will use .iter() and .into_iter() on this
//     // iter() just borrow it, so never change it.
//     let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
//     println!("{:?}", vector1);
//     println!("{:?}", vector1_a);
//
//     // into_iter() will change it ownership
//     let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();
//     // so if we use it now, it will error. (now vector1 is disappear)
//     // println!("{:?}", vector1);
//     println!("{:?}", vector1_b);
//
//     let mut vector2 = vec![10, 20, 30]; // we will use .iter_mut() on this one
//     // we can see don't have any variable to receive it. so iter_mut() will change it on itself.
//     vector2.iter_mut().for_each(|x| *x += 100);
//
//     println!("{:?}", vector2);
// }

// fn main() {
//     let my_vec = vec!['a', 'b', '거', '柳']; // Just a regular Vec
//
//     // here use iter(), so just borrow.
//     let mut my_vec_iter = my_vec.iter(); // This is an Iterator type now, but we haven't called it yet
//
//     assert_eq!(my_vec_iter.next(), Some(&'a'));  // Call the first item with .next()
//     assert_eq!(my_vec_iter.next(), Some(&'b'));  // Call the next
//     assert_eq!(my_vec_iter.next(), Some(&'거')); // Again
//     assert_eq!(my_vec_iter.next(), Some(&'柳')); // Again
//     assert_eq!(my_vec_iter.next(), None);        // Nothing is left: just None
//     assert_eq!(my_vec_iter.next(), None);        // You can keep calling .next() but it will always be None
// }

#[derive(Debug)] // we want to print it with {:?}
struct Library {
    library_type: LibraryType, // this is our enum
    books: Vec<String>,        // list of books
}

#[derive(Debug)]
enum LibraryType {
    // libraries can be city libraries or country libraries
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        // we use add_book to add new books
        self.books.push(book.to_string()); // we take a &str and turn it into a String, then add it to the Vec
    }

    fn new() -> Self {
        // this creates a new Library
        Self {
            library_type: LibraryType::City, // most are in the city so we'll choose City
            // most of the time
            books: Vec::new(),
        }
    }
}

impl Iterator for Library {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.books.pop() {
            Some(book) => Some(book + " is found"),
            None => None,
        }
    }
}

fn main() {
    let mut my_library = Library::new(); // make a new library
    my_library.add_book("The Doom of the Darksword"); // add some books
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");

    println!("{:?}", my_library.books); // we can print our list of books

    for item in my_library {
        println!("{item}");
    }

    // Here is really important thing, when we use for loop,
    // actually we use the into_iter() function. (so we change the ownership)
    let my_vec = vec![21, 3, 43, 2];

    for num in my_vec {
        println!("{num}");
    }
    // that common code could prove it, because will be error.
    // that means we can't use my_vec anymore.
    // for num in my_vec {
    //     println!("{num}");
    // }
}
