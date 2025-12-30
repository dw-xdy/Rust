fn main() {
    let name = "서태지"; // This is a Korean name. No problem, because a &str is UTF-8.
    let other_name = String::from("Adrian Fahrenheit Țepeș"); // Ț and ș are no problem in UTF-8.
    let n = String::from("dskoafnsdhfoiandfsaifa");
    let other_string = "时间".to_string();

    let name = "😂";
    println!("My name is actually {name}");

    println!(
        "A String is always {:?} bytes. It is Sized.",
        size_of::<String>()
    ); // size_of::<Type>() gives you the size in bytes of a type
    println!(
        "And an i8 is always {:?} bytes. It is Sized.",
        size_of::<i8>()
    );
    println!(
        "And an i128 is always {:?} bytes. It is Sized.",
        size_of::<i128>()
    );
    println!(
        "And an f64 is always {:?} bytes. It is Sized.",
        size_of::<f64>()
    );
    println!(
        "But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.",
        size_of_val("서태지")
    ); // size_of_val() gives you the size in bytes of a variable
    println!(
        "And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.",
        size_of_val("Adrian Fahrenheit Țepeș")
    );

    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";

    let together = format!("I am {my_name} and I come from {my_country} but I live in {my_home}.");

    println!("{together}");
}
