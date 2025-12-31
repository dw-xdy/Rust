// fn prints_number(number: i32) { // There is no -> so it's not returning anything
//     // If number was not copy type, it would take it
//     // and we couldn't use it again
//     println!("{}", number);
// }
//
// fn main() {
//     let my_number = 8;
//     prints_number(my_number); // Prints 8. prints_number gets a copy of my_number
//     prints_number(my_number); // Prints 8 again.
//     // No problem, because my_number is copy type!
// }
fn prints_country(country_name: String) {
    println!("{}", country_name);
}

fn main() {
    let country = String::from("Kiribati");
    prints_country(country.clone()); // make a clone and give it to the function. Only the clone goes in, and country is still alive
    prints_country(country);
    // prints_country(country); // but don't change it (⚠️)
}
