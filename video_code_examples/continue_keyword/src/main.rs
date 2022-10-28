fn main() {
    let mut counter = 0;
    println!("Loop started!");

    loop {
        if counter >= 10 {
            break;
        }
        counter += 1;
        if counter % 2 == 0 {
            continue;
        }
        println!("counter: {}", counter);
    }

    println!("Loop finished!");
    println!("Program terminated!");
}