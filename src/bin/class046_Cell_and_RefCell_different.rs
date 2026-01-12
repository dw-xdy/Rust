use std::cell::RefCell;
use std::cell::{Cell, Ref};

#[derive(Debug)]
struct Book<'a> {
    name: Cell<&'a str>,
    author: RefCell<&'a str>,
}

fn main() {
    let my_book = Book {
        name: Cell::new("白夜"),
        author: RefCell::new("陀思妥耶夫斯基"),
    };

    // The take() means: take the value, and just give you default value.
    my_book.name.take();
    my_book.author.take();
    print!("{:?}", my_book);
}
