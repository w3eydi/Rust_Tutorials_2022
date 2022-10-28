fn main() {
    let mut counter = 0;
    println!("Loop started!");
    loop {
        counter += 1;
        println!("counter: {}", counter);

        if counter >= 17 {
            break;
        }
    }
    println!("Loop finished!");
    println!("Program terminated!");
}
