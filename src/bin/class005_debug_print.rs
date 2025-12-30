fn main() {
    let my_number = {
        let second_number = 8;
        second_number + 9;
        // It works just like a function
    };

    // debug print, {:?} print a line, {:#?} print many lines (more readable)
    println!("My number is: {:?}", my_number);
    println!("My number is: {:#?}", my_number);
}
