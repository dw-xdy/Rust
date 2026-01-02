// // create the enum with two choices
// enum ThingsInTheSky {
//     Sun,
//     Stars,
// }
//
// // With this function we can use an i32 to create ThingsInTheSky.
// fn create_skystate(time: i32) -> ThingsInTheSky {
//     match time {
//         6..=18 => ThingsInTheSky::Sun, // Between 6 and 18 hours we can see the sun
//         _ => ThingsInTheSky::Stars, // Otherwise, we can see stars
//     }
// }
//
// // With this function we can match against the two choices in ThingsInTheSky.
// fn check_skystate(state: &ThingsInTheSky) {
//     match state {
//         ThingsInTheSky::Sun => println!("I can see the sun!"),
//         ThingsInTheSky::Stars => println!("I can see the stars!")
//     }
// }
//
// fn main() {
//     let time = 8; // it's 8 o'clock
//     let skystate = create_skystate(time); // create_skystate returns a ThingsInTheSky
//     check_skystate(&skystate); // Give it a reference so it can read the variable skystate
// }

// enum ThingsInTheSky {
//     // we add a String.
//     Sun(String), // Now each variant has a string
//     Stars(String),
// }
//
// fn create_skystate(time: i32) -> ThingsInTheSky {
//     match time {
//         6..=18 => ThingsInTheSky::Sun(String::from("I can see the sun!")), // Write the strings here
//         _ => ThingsInTheSky::Stars(String::from("I can see the stars!")),
//     }
// }
//
// fn check_skystate(state: &ThingsInTheSky) {
//     match state {
//         ThingsInTheSky::Sun(day) => println!("{day}"), // Give the string the name description so we can use it
//         ThingsInTheSky::Stars(night) => println!("{night}"), // Or you can name it n. Or anything else - it doesn't matter
//     }
// }
//
// fn main() {
//     let time = 8; // it's 8 o'clock
//     let skystate = create_skystate(time); // create_skystate returns a ThingsInTheSky
//     check_skystate(&skystate); // Give it a reference so it can read the variable skystate
// }

// enum Mood {
//     Happy,
//     Sleepy,
//     NotBad,
//     Angry,
// }
//
// // fn match_mood(mood: &Mood) -> i32 {
// //     // we don't need this variable, because Rust is really clever.
// //     let happiness_level = match mood {
// //         Mood::Happy => 10, // Here we type Mood:: every time
// //         Mood::Sleepy => 6,
// //         Mood::NotBad => 7,
// //         Mood::Angry => 2,
// //     };
// //     happiness_level
// // }
// fn match_mood(mood: &Mood) -> i32 {
//     use Mood::*;       // use like import , and * also means: everything inside.
//     match mood {
//         Happy => 10, // Here we type Mood:: every time
//         Sleepy => 6,
//         NotBad => 7,
//         Angry => 2,
//     }
// }
//
// fn main() {
//     let my_mood = Mood::NotBad;
//     let happiness_level = match_mood(&my_mood);
//     println!("Out of 1 to 10, my happiness is {happiness_level}");
// }

// enum Something {
//     // every thing in enum, the compiler will give them a secret number.
//     // start from 0;
//     one,
//     two,
// }
//
//
// fn main() {
//     println!("{}", Something::one as u32);
//     println!("{}", Something::two as u32);
// }

// enum Season {
//     Spring, // If this was Spring(String) or something it wouldn't work
//     Summer,
//     Autumn,
//     Winter,
// }
//
// fn main() {
//     use Season::*;
//     let four_seasons = vec![Spring, Summer, Autumn, Winter];
//     for season in four_seasons {
//         println!("{}", season as u32);
//     }
// }

// error!!!!!!!!
// enum Season {
//     // because that's not Primitive data(基元数据)
//     Spring(String), // If this was Spring(String) or something it wouldn't work
//     Summer(String),
//     Autumn(String),
//     Winter(String),
// }
//
// fn main() {
//     use Season::*;
//     let my_str = String::from("enough");
//     let four_seasons = vec![Spring(my_str), Summer(my_str), Autumn(my_str), Winter(my_str)];
//     for season in four_seasons {
//         println!("{}", season as u32);
//     }
// }

// enum Star {
//     BrownDwarf = 10,
//     RedDwarf = 80,
//     YellowStar = 100,
//     RedGiant = 1000,
//     DeadStar, // Think about this one. What number will it have?
// }
//
// fn main() {
//     use Star::*;
//     let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant];
//     for star in starvec {
//         match star as u32 {
//             size if size <= 80 => println!("Not the biggest star."), // Remember: size doesn't mean anything. It's just a name we chose so we can print it
//             size if size > 80 => println!("This is a good-sized star."),
//             _ => println!("That star is pretty big!"),
//         }
//     }
//     println!("What about DeadStar? It's the number {}.", DeadStar as u32);
// }

enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(input: i32) -> Number {
    match input.is_positive() {
        true => Number::U32(input as u32), // change it to u32 if it's positive
        false => Number::I32(input), // otherwise just give the number because it's already i32
    }
}

fn main() {
    let my_vec = vec![get_number(-800), get_number(8)];

    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value {number}"),
            Number::I32(number) => println!("It's an i32 with the value {number}"),
        }
    }
}
