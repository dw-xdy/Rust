 // The signed integers are: i8, i16, i32, i64, i128, and isize. 
 // i 表示对应的有符号值
 // The unsigned integers are: u8, u16, u32, u64, u128, and usize.
 // u 表示没有符号的值.

fn main() {
    let first_letter = 'A';
    let space = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are chars too
    println!("{}", cat_face.len_utf8());

    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    // 可以简写.
    // println!("Size of a char: {}", size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());


    // 依然返回的是字节数. (bytes)
    let slice = "Hello!";
    println!("Slice is {} bytes.", slice.len());
    let slice2 = "안녕!"; // Korean for "hi"
    println!("Slice2 is {} bytes.", slice2.len());

    let slice = "Hello!";
    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}
