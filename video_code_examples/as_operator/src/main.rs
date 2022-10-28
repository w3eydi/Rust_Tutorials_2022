fn main() {
    let float_type = 110.14159_f32;
    let integer_type: u8 = float_type as u8;
    let char_type = integer_type as char;

    println!("float: {}, integer: {}", float_type, integer_type);
    println!("hexadecimal: {:x}, char: {}", integer_type,
            char_type);
    
}