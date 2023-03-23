fn main() {
    let value_x = 5;
    let reference_x = &value_x;

    println!("{}", value_x == *reference_x);
}