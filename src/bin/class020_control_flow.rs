fn main() {
    // if-else don't have any different with Java or C/C++ (just need lost ())
    // so the true important thing is: match (keyword) like C/C++ and Java's switch.
    let my_number: u8 = 9;

    // match also need cover all situation!!!
    match my_number {
        9 => println!("is 9"),
        _ => println!("not 9"), // _ like else(keyword)
    }

    // shadowing!!!
    let my_number = 5;
    let second_number = match my_number {
        0 => 0,
        5 => 10,
        _ => 2,
    };
    println!("{second_number}");

    // also we can match tuple
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => {
            println!("It's dark and unpleasant today")
        }
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", "warm") => {
            println!("It's dark but not bad")
        }
        _ => println!("Not sure what the weather is."),
    }

    let children = 5;
    let married = true;

    match (children, married) {
        (children, married) if married == false => {
            println!("Not married with {} children", children)
        }
        (children, married) if children == 0 && married == true => {
            println!("Married but no children")
        }
        _ => println!("Married? {}. Number of children: {}.", married, children),
    }

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);

    match_number(50);
    match_number(13);
    match_number(4);
}

fn match_colours(rbg: (i32, i32, i32)) {
    match rbg {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, b, _) if b < 10 => println!("Not much blue"),
        (_, _, g) if g < 10 => println!("Not much green"),
        _ => println!("Each colour has at least 10"),
    }
}

fn match_number(input: i32) {
    match input {
        // just like as (keyword)
        number @ 4 => println!("{number} is an unlucky number in China (sounds close to 死)!",),
        number @ 13 => {
            println!("{number} is unlucky in North America, lucky in Italy! In bocca al lupo!",)
        }
        _ => println!("Looks like a normal number"),
    }
}
