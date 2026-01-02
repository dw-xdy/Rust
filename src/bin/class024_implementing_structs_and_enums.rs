// #[derive(Debug)]
// struct Animal {
//     age: u8,
//     animal_type: AnimalType,
// }
//
// #[derive(Debug)]
// enum AnimalType {
//     Cat,
//     Dog,
// }
//
// impl Animal {
//     fn new() -> Self {
//         // Self means Animal.
//         // You can also write Animal instead of Self
//
//         Self {
//             // When we write Animal::new(), we always get a cat that is 10 years old
//             age: 10,
//             animal_type: AnimalType::Cat,
//         }
//     }
//
//     fn change_to_dog(&mut self) {
//         // because we are inside Animal, &mut self means &mut Animal
//         // use .change_to_dog() to change the cat to a dog
//         // with &mut self we can change it
//         println!("Changing animal to dog!");
//         self.animal_type = AnimalType::Dog;
//     }
//
//     fn change_to_cat(&mut self) {
//         // use .change_to_cat() to change the dog to a cat
//         // with &mut self we can change it
//         println!("Changing animal to cat!");
//         self.animal_type = AnimalType::Cat;
//     }
//
//     fn check_type(&self) {
//         // we want to read self
//         match self.animal_type {
//             AnimalType::Dog => println!("The animal is a dog"),
//             AnimalType::Cat => println!("The animal is a cat"),
//         }
//     }
// }
//
// fn main() {
//     let mut new_animal = Animal::new(); // Associated function to create a new animal
//     // It is a cat, 10 years old
//     new_animal.check_type();
//     new_animal.change_to_dog();
//     new_animal.check_type();
//     new_animal.change_to_cat();
//     new_animal.check_type();
// }

// struct Book {
//     number: u32,
// }
//
// impl Book {
//     fn get_number(&self) -> u32 {
//         self.number
//     }
//
//     fn change_number(& mut self, new_number: u32) {
//         self.number = new_number;
//     }
//
//     // new is not a keyword in Rust.
//     // so static method not have self (keyword).
//     fn new(number: u32) -> Self {
//         Self { number }
//     }
// }
//
// fn main() {
//     let mut my_book = Book::new(39);
//
//     println!("{}", my_book.get_number());
//     my_book.change_number(437628);
//     println!("{}", my_book.get_number());
// }

// implement enum
enum Mood {
    Good,
    Bad,
    Sleepy,
}

impl Mood {
    fn check(&self) {
        match self {
            Mood::Good => println!("Feeling good!"),
            Mood::Bad => println!("Eh, not feeling so good"),
            Mood::Sleepy => println!("Need sleep NOW"),
        }
    }
}

fn main() {
    let my_mood = Mood::Sleepy;
    my_mood.check();
}
