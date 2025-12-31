// fn main() {
//     let mut my_number = 8;
//     let num_ref = &mut my_number;
//     *num_ref += 10; // Use * to change the i32 value.
//     println!("{}", my_number);
//
//     let second_number = 800;
//     let triple_reference = &&&second_number;
//     println!("Second_number = triple_reference? {}", second_number == ***triple_reference);
// }

// there are two rules
// 1. immutable reference have many as you want. all the fine
// 2. mutable reference just have one!
// and immutable and mutable can't have together.
// this is an example.
// fn main() {
//     let mut number = 10;
//     let number_ref = &number;
//     let number_change = &mut number;
//     // error[E0502]: cannot borrow `number` as mutable because it is also borrowed as immutable
//     //     --> src\bin\class013_mulable_referece.rs:20:25
//     //     |
//     //     19 |     let number_ref = &number;
//     // |                      ------- immutable borrow occurs here
//     // 20 |     let number_change = &mut number;
//     // |                         ^^^^^^^^^^^ mutable borrow occurs here
//     // 21 |     *number_change += 10;
//     // 22 |     println!("{}", number_ref); // ⚠️
//     // |                    ---------- immutable borrow later used here
//     *number_change += 10;
//     println!("{}", number_ref); // ⚠️
// }
// maybe we can do this: just use immutable later mutable.
fn main() {
    let mut number = 10;
    let number_change = &mut number;

    *number_change += 10;
    let number_ref = &number;
    println!("{}", number_ref); // that's ok.
}
