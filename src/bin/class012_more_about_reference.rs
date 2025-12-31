fn return_str() -> String {
    let country = String::from("Austria");
    country // ⚠️
}

fn main() {
    // the reference: who owns a value
    let country = return_str();
    println!("{country}");
}
