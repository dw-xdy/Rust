 // The signed integers are: i8, i16, i32, i64, i128, and isize. 
 // i 表示对应的有符号值
 // The unsigned integers are: u8, u16, u32, u64, u128, and usize.
 // u 表示没有符号值.
 
fn main() {
    let first_letter = 'A';
    let space = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are chars too
    println!("{}", cat_face);
}
