// more about destructuring
struct Person { // make a simple struct for a person
    name: String,
    real_name: String,
    height: u8,
    happiness: bool
}

fn main() {
    // here we create a Person and named papa_doc
    let papa_doc = Person { // create variable papa_doc
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false
    };


    // now we destructuring papa_doc and assign to Person.
    // and a, b, c, d is renamed the Person's name.
    let Person { // destructure papa_doc
        name: a,
        real_name: b,
        // height: c,
        // happiness: d
        // if we don't care anything else.
        ..
    } = papa_doc;

    // println!("They call him {a} but his real name is {b}. He is {c} cm tall and is he happy? {d}");
    println!("They call him {a} but his real name is {b}.");

}