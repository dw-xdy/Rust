// four method we usually use.
// is_some()
// is_none()
// is_ok()
// is_err()
fn main() {
    let my_vec = vec![2_u32, 3, 4];

    // that's means: we don't care about other situations.
    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {number}");
        }
    }
}
