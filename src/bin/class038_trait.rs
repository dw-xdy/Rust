// struct Animal { // A simple struct - an Animal only has a name
//     name: String,
// }
//
//
// // The trait just like Java's interface.
// // trait just like power, that make Rust more flexible.
// trait Dog { // The dog trait gives some functionality
//     fn bark(&self) { // It can bark
//         println!("Woof woof!");
//     }
//     fn run(&self) { // and it can run
//         println!("The dog is running!");
//     }
// }
//
// impl Dog for Animal {  // Now Animal has the trait Dog
//     fn run(&self) {
//         println!("The dog is sparking");
//     }
// }
//
// fn main() {
//     let rover = Animal {
//         name: "Rover".to_string(),
//     };
//
//     rover.bark(); // Now Animal can use bark()
//     rover.run();  // and it can use run()
// }

// #[derive(Debug)]
// struct Cat {
//     name: String,
//     age: u8,
// }
//
// use std::fmt;
// impl fmt::Display for Cat {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let cat_type = match self.age {
//             0..=2 => "kitten",
//             3..=10 => "adult cat",
//             _ => "old cat",
//         };
//
//         write!(
//             f,
//             "{} is a cat who is {} years old, it a {}",
//             self.name, self.age, cat_type
//         )
//     }
// }
//
// fn main() {
//     let mr_mantle = Cat {
//         name: "Reggie Mantle".to_string(),
//         age: 4,
//     };
//
//     println!("{}", mr_mantle);
// }

// struct Monster {
//     health: i32,
// }
//
// struct Wizard {}
// struct Ranger {}
//
// trait FightClose {
//     fn attack_with_sword(&self, opponent: &mut Monster) {
//         opponent.health -= 10;
//         println!(
//             "You attack with your sword. Your opponent now has {} health left.",
//             opponent.health
//         );
//     }
//     fn attack_with_hand(&self, opponent: &mut Monster) {
//         opponent.health -= 2;
//         println!(
//             "You attack with your hand. Your opponent now has {} health left.",
//             opponent.health
//         );
//     }
// }
// impl FightClose for Wizard {}
// impl FightClose for Ranger {}
//
// trait FightFromDistance {
//     fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
//         if distance < 10 {
//             opponent.health -= 10;
//             println!(
//                 "You attack with your bow. Your opponent now has {} health left.",
//                 opponent.health
//             );
//         }
//     }
//     fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
//         if distance < 3 {
//             opponent.health -= 4;
//         }
//         println!(
//             "You attack with your rock. Your opponent now has {} health left.",
//             opponent.health
//         );
//     }
// }
// impl FightFromDistance for Ranger {}
//
// fn main() {
//     let radagast = Wizard {};
//     let aragorn = Ranger {};
//
//     let mut uruk_hai = Monster { health: 40 };
//
//     radagast.attack_with_sword(&mut uruk_hai);
//     aragorn.attack_with_bow(&mut uruk_hai, 8);
// }

// struct Monster {
//     health: i32,
// }
//
// #[derive(Debug)] // Now Wizard has Debug
// struct Wizard {
//     health: i32, // Now Wizard has health
// }
// #[derive(Debug)] // So does Ranger
// struct Ranger {
//     health: i32, // So does Ranger
// }
//
// trait FightClose: std::fmt::Debug { // Now a type needs Debug to use FightClose
//     fn attack_with_sword(&self, opponent: &mut Monster) {
//         opponent.health -= 10;
//         println!(
//             "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}", // We can now print self with {:?} because we have Debug
//             opponent.health, &self
//         );
//     }
//     fn attack_with_hand(&self, opponent: &mut Monster) {
//         opponent.health -= 2;
//         println!(
//             "You attack with your hand. Your opponent now has {} health left.  You are now at: {:?}",
//             opponent.health, &self
//         );
//     }
// }
// impl FightClose for Wizard {}
// impl FightClose for Ranger {}
//
// trait FightFromDistance: std::fmt::Debug { // We could also do trait FightFromDistance: FightClose because FightClose needs Debug
//     fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
//         if distance < 10 {
//             opponent.health -= 10;
//             println!(
//                 "You attack with your bow. Your opponent now has {} health left.  You are now at: {:?}",
//                 opponent.health, self
//             );
//         }
//     }
//     fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
//         if distance < 3 {
//             opponent.health -= 4;
//         }
//         println!(
//             "You attack with your rock. Your opponent now has {} health left.  You are now at: {:?}",
//             opponent.health, self
//         );
//     }
// }
// impl FightFromDistance for Ranger {}
//
// fn main() {
//     let radagast = Wizard { health: 60 };
//     let aragorn = Ranger { health: 80 };
//
//     let mut uruk_hai = Monster { health: 40 };
//
//     radagast.attack_with_sword(&mut uruk_hai);
//     aragorn.attack_with_bow(&mut uruk_hai, 8);
// }

// use std::fmt::Debug; // So we don't have to write std::fmt::Debug every time now
//
// struct Monster {
//     health: i32,
// }
//
// #[derive(Debug)]
// struct Wizard {
//     health: i32,
// }
// #[derive(Debug)]
// struct Ranger {
//     health: i32,
// }
//
// trait Magic {} // No methods for any of these traits. They are just trait bounds
// trait FightClose {}
// trait FightFromDistance {}
//
// impl FightClose for Ranger {} // Each type gets FightClose,
// impl FightClose for Wizard {}
// impl FightFromDistance for Ranger {} // but only Ranger gets FightFromDistance
// impl Magic for Wizard {} // and only Wizard gets Magic
//
// fn attack_with_bow<T: FightFromDistance + Debug>(
//     character: &T,
//     opponent: &mut Monster,
//     distance: u32,
// ) {
//     if distance < 10 {
//         opponent.health -= 10;
//         println!(
//             "You attack with your bow. Your opponent now has {} health left.  You are now at: {:?}",
//             opponent.health, character
//         );
//     }
// }
//
// fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
//     opponent.health -= 10;
//     println!(
//         "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
//         opponent.health, character
//     );
// }
//
// fn fireball<T: Magic + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
//     if distance < 15 {
//         opponent.health -= 20;
//         println!(
//             "You raise your hands and cast a fireball! Your opponent now has {} health left. You are now at: {:?}",
//             opponent.health, character
//         );
//     }
// }
//
// fn main() {
//     let radagast = Wizard { health: 60 };
//     let aragorn = Ranger { health: 80 };
//
//     let mut uruk_hai = Monster { health: 40 };
//
//     attack_with_sword(&radagast, &mut uruk_hai);
//     attack_with_bow(&aragorn, &mut uruk_hai, 8);
//     fireball(&radagast, &mut uruk_hai, 8);
// }

//
// use std::fmt::Display; // We will make a generic function to print them so we want Display
//
// fn print_vec<T: Display>(input: &Vec<T>) { // Take any Vec<T> if type T has Display
//     for item in input {
//         print!("{} ", item);
//     }
//     println!();
// }
//
// fn main() {
//
//     let array_vec = Vec::from([8, 9, 10]); // Try from an array
//     print_vec(&array_vec);
//
//     let str_vec = Vec::from("What kind of vec will I be?"); // An array from a &str? This will be interesting
//     print_vec(&str_vec);
//
//     // Here is very interesting thing, "你": 228, 189, 160: that's utf-8 because it's char(i32)
//     let string_chinese_vec = Vec::from("你hat kind of vec will a String be?".to_string()); // Also from a String
//     print_vec(&string_chinese_vec);
//
//     let string_vec = Vec::from("What kind of vec will a String be?".to_string()); // Also from a String
//     print_vec(&string_vec);
// }

// #[derive(Debug)] // So we can print City
// struct City {
//     name: String,
//     population: u32,
// }
//
// // that just as a static method.
// impl City {
//     fn new(name: &str, population: u32) -> Self { // just a new function
//         Self {
//             name: name.to_string(),
//             population,
//         }
//     }
// }
//
// #[derive(Debug)] // Country also needs to be printed
// struct Country {
//     cities: Vec<City>, // Our cities go in here
// }
//
//
// impl From<Vec<City>> for Country { // Note: we don't have to write From<City>, we can also do
//     // From<Vec<City>>. So we can also implement on a type that
//     // we didn't create
//     fn from(cities: Vec<City>) -> Self {
//         Self { cities }
//     }
// }
//
//
// impl Country {
//     fn print_cities(&self) { // function to print the cities in Country
//         for city in &self.cities {
//             // & because Vec<City> isn't Copy
//             println!("{:?} has a population of {:?}.", city.name, city.population);
//         }
//     }
// }
//
// fn main() {
//     let helsinki = City::new("Helsinki", 631_695);
//     let turku = City::new("Turku", 186_756);
//
//     let finland_cities = vec![helsinki, turku]; // This is the Vec<City>
//     let finland = Country::from(finland_cities); // So now we can use From
//
//     finland.print_cities();
// }

use std::fmt::Display;

// fn print_it<T>(input: T) {
//     println!("{}", input);
// }

// use std::convert::From;
//
// struct EvenOddVec(Vec<Vec<i32>>);
//
// impl From<Vec<i32>> for EvenOddVec {
//     fn from(input: Vec<i32>) -> Self {
//         let mut even_odd_vec: Vec<Vec<i32>> = vec![vec![], vec![]]; // A vec with two empty vecs inside
//         // This is the return value, but first we must fill it
//         for item in input {
//             if item % 2 == 0 {
//                 even_odd_vec[0].push(item);
//             } else {
//                 even_odd_vec[1].push(item);
//             }
//         }
//         Self(even_odd_vec) // Now it is done so we return it as Self (Self = EvenOddVec)
//     }
// }
//
// fn main() {
//     let bunch_of_numbers = vec![8, 7, -1, 3, 222, 9787, -47, 77, 0, 55, 7, 8];
//     let new_vec = EvenOddVec::from(bunch_of_numbers);
//
//     println!(
//         "Even numbers: {:?}\nOdd numbers: {:?}",
//         new_vec.0[0], new_vec.0[1]
//     );
// }

// This is really important, because if you don't have Display trait, you can't use println!()
// because the {} need Display trait.
// fn print_it<T: Display>(input: T) {
//     println!("{}", input);
// }
//
// fn main() {
//     print_it("hello world")
// }

fn print_it<T>(input: T)
where
    T: Display + AsRef<str>,
{
    println!("{input}")
}

fn main() {
    print_it("S".to_string());
}
