fn main() {
    // Signed integer maximums
    println!("i8   MAX = {}", i8::MAX);
    println!("i16  MAX = {}", i16::MAX);
    println!("i32  MAX = {}", i32::MAX);
    println!("i64  MAX = {}", i64::MAX);
    println!("i128 MAX = {}", i128::MAX);
    println!("isize MAX = {}", isize::MAX);

    // Unsigned integer maximums
    println!("u8   MAX = {}", u8::MAX);
    println!("u16  MAX = {}", u16::MAX);
    println!("u32  MAX = {}", u32::MAX);
    println!("u64  MAX = {}", u64::MAX);
    println!("u128 MAX = {}", u128::MAX);
    println!("usize MAX = {}", usize::MAX);

    // Floating-point maximums
    println!("f32  MAX = {}", f32::MAX);
    println!("f64  MAX = {}", f64::MAX);

    // Boolean operations
    let a = true;
    let b = false;
    println!("true AND false = {}", a && b);
    println!("true OR false  = {}", a || b);
    println!("NOT true      = {}", !a);
    println!("true XOR false = {}", a ^ b);

    // Character examples
    let letter = 'A';
    let digit = '7';
    let symbol = '#';
    let emoji = '😀';
    let heart = '❤';
    println!("Letter: {}", letter);
    println!("Digit: {}", digit);
    println!("Symbol: {}", symbol);
    println!("Emoji: {}", emoji);
    println!("Heart: {}", heart);
}
