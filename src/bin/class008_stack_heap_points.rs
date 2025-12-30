fn main() {
    let num = 19;
    // this means reference
    let my_num = &num;
    let real_num = &&&&num;

    println!("{}", num);
    print!("{}", my_num);
    print!("{}", real_num);
}