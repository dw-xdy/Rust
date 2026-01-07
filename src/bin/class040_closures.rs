// fn main() {
//     let my_closure = || println!("This is a closure");
//     my_closure();
// }
// fn main() {
//     let my_vec = vec![8, 9, 10];
//
//     let fourth = my_vec.get(3).unwrap_or_else(|| {
//         // try to unwrap. If it doesn't work,
//         if my_vec.get(0).is_some() {
//             // see if my_vec has something at index [0]
//             &my_vec[0] // Give the number at index 0 if there is something
//         } else {
//             &0 // otherwise give a &0
//         }
//     });
//
//     println!("{fourth}");
// }

// fn main() {
//     let x: Option<String> = None;
//
//     println!(
//         "{:?}",
//         x.unwrap_or_else(|| {
//             let mut my_string = "I am but a String".to_string();
//             my_string.push('!');
//             my_string
//         })
//     );
// }

// fn main() {
//     let num_vec = vec![10, 9, 8];
//
//     num_vec
//         .iter() // iterate over num_vec
//         .enumerate() // get (index, number)
//         // .for_each(|(index, number)| println!("Index number {index} has number {number}
//         // that's tuple, so we deconstruct it.
//         .for_each(|(key, value)| println!("Index number {key} has number {value}"));
//         // do something for each one
// }

// use std::collections::HashMap;
//
// fn main() {
//     let some_numbers = vec![0, 1, 2, 3, 4, 5]; // a Vec<i32>
//     let some_words = vec!["zero", "one", "two", "three", "four", "five"]; // a Vec<&str>
//
//     let number_word_hashmap = some_numbers
//         .into_iter() // now it is an iter
//         .zip(some_words.into_iter()) // inside .zip() we put in the other iter. Now they are together.
//         .collect::<HashMap<_, _>>();
//
//     println!(
//         "For key {} we get {}.",
//         &2,
//         number_word_hashmap.get(&2).unwrap()
//     );
//
//     // let map: HashMap<_, _> = some_numbers.iter()      // &i32
//     //     .zip(some_words.iter())                       // &&str
//     //     .map(|(&num, &word)| (num, word))             // 解引用获取值
//     //     .collect();
//     //
//     // println!("{:?}", map);
// }

// fn main() {
//     let numbers_together = "140399923481800622623218009598281";
//
//     for (index, number) in numbers_together.char_indices() {
//         match (index % 3, number) {
//             (0..=1, number) => print!("{number}"), // just print the number if there is a remainder
//             _ => print!("{number}\t"),             // otherwise print the number with a tab space
//         }
//     }
//
//     println!();
//     println!("{:?}", numbers_together.chars().collect::<String>());
// }

// fn main() {
//     let my_vec = vec![8, 9, 10];
//
//     println!("{:?}", my_vec.iter().for_each(|_| println!("We didn't use the variables at all"))); // ⚠️
// }

// fn main() {
//     let months = vec![
//         "January",
//         "February",
//         "March",
//         "April",
//         "May",
//         "June",
//         "July",
//         "August",
//         "September",
//         "October",
//         "November",
//         "December",
//     ];
//
//     let filtered_months = months
//         .into_iter() // make an iter
//         .filter(|month| month.len() < 5) // We don't want months more than 5 bytes in length.
//         // We know that each letter is one byte so .len() is fine
//         .filter(|month| month.contains("u")) // Also we only like months with the letter u
//         .collect::<Vec<&str>>();
//
//     println!("{:?}", filtered_months);
//
//
//     let mut my_vec = vec![1, 2, 3, 4];
//
//     // The retain() function means make change in origin data.
//     my_vec.retain(|item| item % 2 == 0);
//     println!("{:?}", my_vec);
// }

// struct Company {
//     name: String,
//     ceo: Option<String>,
// }
//
// impl Company {
//     fn new(name: &str, ceo: &str) -> Self {
//         let ceo = match ceo {
//             "" => None,
//             name => Some(name.to_string()),
//         }; // ceo is decided, so now we return Self
//         Self {
//             name: name.to_string(),
//             ceo,
//         }
//     }
//
//     fn get_ceo(&self) -> Option<String> {
//         self.ceo.clone() // Just returns a clone of the CEO (struct is not Copy)
//     }
// }
//
// fn main() {
//     let company_vec = vec![
//         Company::new("Umbrella Corporation", "Unknown"),
//         Company::new("Ovintiv", "Doug Suttles"),
//         Company::new("The Red-Headed League", ""),
//         Company::new("Stark Enterprises", ""),
//     ];
//
//     // That will return the Option, so filter_map() only filter Some(),
//     // if return None, method don't care.
//     let all_the_ceos = company_vec
//         .into_iter()
//         .filter_map(|company| company.get_ceo()) // filter_map needs Option<T>
//         .collect::<Vec<String>>();
//
//     println!("{:?}", all_the_ceos);
// }

// fn main() {
//     let user_input = vec!["8.9", "Nine point nine five", "8.0", "7.6", "eleventy-twelve"];
//
//     let actual_numbers = user_input
//         .into_iter()
//         // .ok() method will make Result enum to Option enum. so we can use filter_map() method.
//         .filter_map(|input| input.parse::<f32>().ok())
//         .collect::<Vec<f32>>();
//
//     println!("{:?}", actual_numbers);
// }

// Everything before main() is exactly the same
// struct Company {
//     name: String,
//     ceo: Option<String>,
// }

//
// impl Company {
//     fn new(name: &str, ceo: &str) -> Self {
//         let ceo = match ceo {
//             "" => None,
//             name => Some(name.to_string()),
//         };
//         Self {
//             name: name.to_string(),
//             ceo,
//         }
//     }
//
//     fn get_ceo(&self) -> Option<String> {
//         self.ceo.clone()
//     }
// }
//
// fn main() {
//     let company_vec = vec![
//         Company::new("Umbrella Corporation", "Unknown"),
//         Company::new("Ovintiv", "Doug Suttles"),
//         Company::new("The Red-Headed League", ""),
//         Company::new("Stark Enterprises", ""),
//     ];
//
//     let mut results_vec = vec![]; // Pretend we need to gather error results too
//
//     // Here we can't use the filter_map() because we need the Result. not Option.
//     // so we use ok_or() and ok_or_else(), these methods make Option -> Result.
//     company_vec
//         .iter()
//         .for_each(|company| results_vec.push(company.get_ceo().ok_or("No CEO found")));
//
//     for item in results_vec {
//         println!("{:?}", item);
//     }
// }

// // Everything before main() is exactly the same
// struct Company {
//     name: String,
//     ceo: Option<String>,
// }
//
// impl Company {
//     fn new(name: &str, ceo: &str) -> Self {
//         let ceo = match ceo {
//             "" => None,
//             name => Some(name.to_string()),
//         };
//         Self {
//             name: name.to_string(),
//             ceo,
//         }
//     }
//
//     fn get_ceo(&self) -> Option<String> {
//         self.ceo.clone()
//     }
// }
//
// fn main() {
//     let company_vec = vec![
//         Company::new("Umbrella Corporation", "Unknown"),
//         Company::new("Ovintiv", "Doug Suttles"),
//         Company::new("The Red-Headed League", ""),
//         Company::new("Stark Enterprises", ""),
//     ];
//
//     let mut results_vec = vec![];
//
//     company_vec.iter().for_each(|company| {
//         results_vec.push(company.get_ceo().ok_or_else(|| {
//             let err_message = format!("No CEO found for {}", company.name);
//             err_message
//         }))
//     });
//
//     for item in results_vec {
//         println!("{:?}", item);
//     }
// }

// fn main() {
//     let new_vec = vec![8, 9, 0]; // just a vec with numbers
//
//     let number_to_add = 5;       // use this in the math later
//     let mut empty_vec = vec![];  // results go in here
//
//
//     for index in 0..5 {
//         empty_vec.push(
//             new_vec
//                 .get(index)
//                 .and_then(|number| Some(number + 1))
//                 .and_then(|number| Some(number + number_to_add))
//         );
//     }
//     println!("{:?}", empty_vec);
// }

// fn main() {
//     let first_try = vec![Some("success!"), None, Some("success!"), Some("success!"), None];
//     let second_try = vec![None, Some("success!"), Some("success!"), Some("success!"), Some("success!")];
//     let third_try = vec![Some("success!"), Some("success!"), Some("success!"), Some("success!"), None];
//
//     for i in 0..first_try.len() {
//         println!("{:?}", first_try[i].and(second_try[i]).and(third_try[i]));
//     }
// }

// fn in_char_vec(char_vec: &Vec<char>, check: char) {
//     println!(
//         "Is {check} inside? {}",
//         char_vec.iter().any(|&char| char == check)
//     );
// }
//
// fn main() {
//     let char_vec = ('a'..'働').collect::<Vec<char>>();
//     in_char_vec(&char_vec, 'i');
//     in_char_vec(&char_vec, '뷁');
//     in_char_vec(&char_vec, '鑿');
//
//     let smaller_vec = ('A'..'z').collect::<Vec<char>>();
//     println!(
//         "All alphabetic? {}",
//         smaller_vec.iter().all(|&x| x.is_alphabetic())
//     );
//     println!(
//         "All less than the character 행? {}",
//         smaller_vec.iter().all(|&x| x < '행')
//     );
// }

// fn main() {
//     let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
//
//     // find() give it for you.
//     println!("{:?}", num_vec.iter().find(|&number| number % 3 == 0)); // find takes a reference, so we give it &number
//     println!("{:?}", num_vec.iter().find(|&number| number * 2 == 30));
//
//     // position() give it position for you.
//     println!("{:?}", num_vec.iter().position(|&number| number % 3 == 0));
//     println!("{:?}", num_vec.iter().position(|&number| number * 2 == 30));
// }

// fn main() {
//     let even_odd = vec!["even", "odd"];
//
//     let even_odd_vec = (0..6)
//         .zip(even_odd.into_iter().cycle()) // cycle() means: never stop.
//         .collect::<Vec<(i32, &str)>>();
//     println!("{:?}", even_odd_vec);
// }

// fn main() {
//     let some_numbers = vec![9, 6, 9, 10, 11];
//
//     println!("{}", some_numbers
//         .iter()
//         .fold(0, |total_so_far, next_number| total_so_far + next_number)
//     );
// }

// fn main() {
//     let a_string = "I don't have any dashes in me.";
//
//     println!(
//         "{}",
//         a_string
//             .chars() // Now it's an iterator
//             .fold("-".to_string(), |mut string_so_far, next_char| { // Start with a String "-". Bring it in as mutable each time along with the next char
//                 string_so_far.push(next_char); // Push the char on, then '-'
//                 string_so_far.push('-');
//                 string_so_far} // Don't forget to pass it on to the next loop
//             ));
// }

// fn main() {
//     let num_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
//
//     for chunk in num_vec.chunks(3) {
//         println!("{:?}", chunk);
//     }
//     println!();
//     for window in num_vec.windows(3) {
//         println!("{:?}", window);
//     }
// }

// fn main() {
//     let rules = "Rule number 1: No fighting. Rule number 2: Go to bed at 8 pm. Rule number 3: Wake up at 6 am.";
//     let rule_locations = rules.match_indices("Rule").collect::<Vec<(_, _)>>(); // This is Vec<usize, &str> but we just tell Rust to do it
//     println!("{:?}", rule_locations);
// }

// fn main() {
//     let just_numbers = vec![1, 5, 100];
//     let mut number_iter = just_numbers.iter().peekable(); // This actually creates a type of iterator called Peekable
//
//     for _ in 0..3 {
//         println!("I love the number {}", number_iter.peek().unwrap());
//         println!("I really love the number {}", number_iter.peek().unwrap());
//         println!("{} is such a nice number", number_iter.peek().unwrap());
//         number_iter.next();
//     }
// }

// fn main() {
//     let locations = vec![
//         ("Nevis", 9),
//         ("Taber", 8428),
//         ("Markerville", 45),
//         ("Cardston", 3585),
//     ];
//
//     let mut location_iter = locations.iter().peekable();
//     while location_iter.peek().is_some() {
//         match location_iter.peek() {
//             Some((name, number)) if *number < 100 => {
//                 // .peek() gives us a reference so we need *
//                 println!("Found a hamlet: {name} with {number} people")
//             }
//             Some((name, number)) => println!("Found a town: {name} with {number} people"),
//             None => break,
//         }
//         location_iter.next();
//     }
// }

#[derive(Debug)]
struct Names {
    one_word: Vec<String>,
    two_words: Vec<String>,
    three_words: Vec<String>,
}

fn main() {
    let vec_of_names = vec![
        "Caesar",
        "Frodo Baggins",
        "Bilbo Baggins",
        "Jean-Luc Picard",
        "Data",
        "Rand Al'Thor",
        "Paul Atreides",
        "Barack Hussein Obama",
        "Bill Jefferson Clinton",
    ];

    let mut iter_of_names = vec_of_names.iter().peekable();

    let mut all_names = Names {
        // start an empty Names struct
        one_word: vec![],
        two_words: vec![],
        three_words: vec![],
    };

    while iter_of_names.peek().is_some() {
        let next_item = iter_of_names.next().unwrap(); // We can use .unwrap() because we know it is Some
        match next_item.match_indices(' ').collect::<Vec<_>>().len() {
            // Create a quick vec using .match_indices and check the length
            0 => all_names.one_word.push(next_item.to_string()),
            1 => all_names.two_words.push(next_item.to_string()),
            _ => all_names.three_words.push(next_item.to_string()),
        }
    }

    println!("{:?}", all_names);
}
