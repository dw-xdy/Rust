fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}

fn main() {
    let my_number;

    {
        // Pretend we need to have this code block
        let number = {
            // Pretend there is code here to make a number
            // Lots of code, and finally:
            57
        };

        my_number = loop_then_return(number);
    }

    println!("{}", my_number);
}
