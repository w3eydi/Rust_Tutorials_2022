#[allow(unused_variables)] // Use only when you are going to experiment.
fn main() {
    let number_one: i8 = 27;
    let number_two: i8 = 5;

    let addition: i32 = 100 + 5;
    let subtraction: i16 = 20 - 7;
    let multiplication: u8 = 15 * 3;
    let division = number_one / number_two;
    
    let remainder = number_one % number_two;
    
    println!("remainder(mod): {}", remainder);
}
