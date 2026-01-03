use std::collections::HashMap; // This is so we can just write HashMap instead of std::collections::HashMap every time

// // The HashMap.
// struct City {
//     name: String,
//     population: HashMap<u32, u32>, // This will have the year and the population for the year
// }
//
// fn main() {
//     let mut tallinn = City {
//         name: "Tallinn".to_string(),
//         population: HashMap::new(), // So far the HashMap is empty
//     };
//
//     tallinn.population.insert(1372, 3_250); // insert three dates
//     tallinn.population.insert(1851, 24_000);
//     tallinn.population.insert(2020, 437_619);
//
//     for (year, population) in tallinn.population {
//         // The HashMap is HashMap<u32, u32> so it returns two items each time
//         println!(
//             "In the year {year} the city of {} had a population of {population}.",
//             tallinn.name
//         );
//     }
// }

// // The B-TreeMap
// use std::collections::BTreeMap; // Just change HashMap to BTreeMap
//
// struct City {
//     name: String,
//     population: BTreeMap<u32, u32>, // Just change HashMap to BTreeMap
// }
//
// fn main() {
//     let mut tallinn = City {
//         name: "Tallinn".to_string(),
//         population: BTreeMap::new(), // Just change HashMap to BTreeMap
//     };
//
//     tallinn.population.insert(1372, 3_250);
//     tallinn.population.insert(1851, 24_000);
//     tallinn.population.insert(2020, 437_619);
//
//     for (year, population) in tallinn.population {
//         println!(
//             "In the year {year} the city of {} had a population of {population}.",
//             tallinn.name
//         );
//     }
// }

// fn main() {
//     let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
//     let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];
//
//     let mut city_hashmap = HashMap::new();
//
//     // different Key, but same Value.
//     for city in canadian_cities {
//         city_hashmap.insert(city, "Canada");
//     }
//
//     // different Key, but same Value.
//     for city in german_cities {
//         city_hashmap.insert(city, "Germany");
//     }
//
//     println!("{:?}", city_hashmap["Bielefeld"]);
//     println!("{:?}", city_hashmap.get("Bielefeld"));
//     println!("{:?}", city_hashmap.get("Bielefeldd"));
//
//     if let Some(city_name) = city_hashmap.get("Bielefeld") {
//         println!("{city_name}")
//     }
// }

// fn main() {
//     let mut book_hashmap = HashMap::new();
//
//     book_hashmap.insert(1, "L'Allemagne Moderne");
//     book_hashmap.insert(1, "Le Petit Prince");
//     book_hashmap.insert(1, "섀도우 오브 유어 스마일");
//     book_hashmap.insert(1, "Eye of the World");
//
//     // For HashMap, we need use &.
//     println!("{:?}", book_hashmap.get(&1));
// }

// fn main() {
//     let book_collection = vec![
//         "L'Allemagne Moderne",
//         "Le Petit Prince",
//         "Eye of the World",
//         "Eye of the World",
//     ]; // Eye of the World appears twice
//
//     let mut book_hashmap = HashMap::new();
//
//     for book in book_collection {
//         book_hashmap.entry(book).or_insert(true);
//     }
//
//     for (book, true_or_false) in book_hashmap {
//         println!("Do we have {book}? {true_or_false}");
//     }
// }

// fn main() {
//     let book_collection = vec![
//         "L'Allemagne Moderne",
//         "Le Petit Prince",
//         "Eye of the World",
//         "Eye of the World",
//     ];
//
//     let mut book_hashmap = HashMap::new();
//
//     for book in book_collection {
//         // let return_value = book_hashmap.entry(book).or_insert(0);
//         // // return_value is a mutable reference. If nothing is there, it will be 0
//         // *return_value += 1; // Now return_value is at least 1. And if there was another book, it will go up by 1
//
//         *book_hashmap.entry(book).or_insert(0) += 1;
//     }
//
//     for (book, number) in book_hashmap {
//         println!("{book}, {number}");
//     }
// }

fn main() {
    let data = vec![
        // This is the raw data
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];

    let mut survey_hash = HashMap::new();

    for item in data {
        // This gives a tuple of (&str, i32)
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
        // This pushes the number into the Vec inside
    }

    for (male_or_female, numbers) in survey_hash {
        println!("{:?}: {:?}", male_or_female, numbers);
    }
}
