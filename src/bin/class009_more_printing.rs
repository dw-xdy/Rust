fn main() {
    // Note: After the first line you have to start on the far left.
    // If you write directly under println!, it will add the spaces
    println!(
        "Inside quotes
you can write over
many lines
and it will print just fine."
    );

    println!(
        "If you forget to write
    on the left side, the spaces
    will be added when you print."
    );

    // two println! will print same thing to console
    println!(
        "He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file."
    ); // We used \ five times here
    // this str's deal like python's r"", only add two # to begin and end.
    println!(
        r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#
    );

    // this is very important question
    /* if you have a # in the str,
    then you must add ##(just == #'s count + 1 in str) to begin and end*/
    let my_string = "'Ice to see you,' he said."; // single quotes
    let quote_string = r#""Ice to see you," he said."#; // double quotes
    let hashtag_string = r##"The hashtag #IceToSeeYou had become very popular."##; // Has one # so we need at least ##
    let many_hashtags =
        r####""You don't have to type ### to use a hashtag. You can just use #.""####; // Has three ### so we need at least ####

    println!(
        "{}\n{}\n{}\n{}\n",
        my_string, quote_string, hashtag_string, many_hashtags
    );

    // we need notice: just ASCII, if you want print chinese or other
    // you need conversion it to UTF-8(And it needs to be in hexadecimal).
    println!("{:?}", b"This will look like numbers");
    println!("{:#?}", b"This will look like numbers");

    println!("{:X}", '행' as u32); // Cast char as u32 to get the hexadecimal value
    println!("{:X}", 'H' as u32);
    println!("{:X}", '居' as u32);
    println!("{:X}", 'い' as u32);

    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}"); // Try printing them with unicode escape \u

    let number = 9;
    let number_ref = &number;
    // {:p} means print number's address in computer's memory.
    println!("{:p}", number_ref);

    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Țepeș";
    // use number in {}, it can specify order
    println!(
        "This is {1} {2}, son of {0} {2}.",
        father_name, son_name, family_name
    );

    println!(
        "{city1} is in {country} and {city2} is also in {country},
but {city3} is not in {country}.",
        city1 = "Seoul",
        city2 = "Busan",
        city3 = "Tokyo",
        country = "Korea"
    );

    let num = 1;
    println!("{num}");

    let letter = "a";
    // left
    println!("{:ㅎ<11}", letter);
    // middle
    println!("{:ㅎ^11}", letter);
    // right
    println!("{:ㅎ>11}", letter);
}
