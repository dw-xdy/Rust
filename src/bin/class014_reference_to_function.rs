// fn print_country(country_name: String) {
//     println!("{}", country_name);
// }
//
// fn main() {
//     let country = String::from("Austria");
//     print_country(country); // We print "Austria"
//     print_country(country); // ⚠️ That was fun, let's do it again!
// }

// // we can fix that error like this way.
// fn print_country(country_name: String) -> String {
//     println!("{}", country_name);
//     country_name // return it here
// }
//
// fn main() {
//     let country = String::from("Austria");
//     let country = print_country(country); // we have to use let here now to get the String back
//     print_country(country);
// }

// // but this way much better.
// fn print_country(country_name: &String) {
//     println!("{}", country_name);
// }
//
// fn main() {
//     let country = String::from("Austria");
//     print_country(&country); // We print "Austria"
//     print_country(&country); // That was fun, let's do it again!
// }

// this example just like owner change adds_hungary function.
// so don't think it's a &String it's not!
// we need know about these are different!.
fn main() {
    let country = String::from("Austria"); // country is not mutable, but we are going to print Austria-Hungary. How?
    adds_hungary(country);
}

fn adds_hungary(mut country: String) {
    // Here's how: adds_hungary takes the String and declares it mutable!
    country.push_str("-Hungary");
    println!("{}", country);
}
