struct Animal { // A simple struct - an Animal only has a name
    name: String,
}


// The trait just like Java's interface.
// trait just like power, that make Rust more flexible.
trait Dog { // The dog trait gives some functionality
    fn bark(&self) { // It can bark
        println!("Woof woof!");
    }
    fn run(&self) { // and it can run
        println!("The dog is running!");
    }
}

impl Dog for Animal {  // Now Animal has the trait Dog
    fn run(&self) {
        println!("The dog is sparking");
    }
}

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };

    rover.bark(); // Now Animal can use bark()
    rover.run();  // and it can use run()
}
