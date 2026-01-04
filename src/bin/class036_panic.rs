// fn main() {
//     panic!("Time to panic!");
// }

fn prints_three_things(vector: Vec<i32>) {
    if vector.len() != 3 {
        panic!("my_vec must always have three items") // will panic if the length is not 3
    }
    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

fn main() {
    let my_vec = vec![8, 9, 10];
    prints_three_things(my_vec);
}
