fn main() {
    for index in 1..=10 {
        if index == 5 {
            continue;
        }

        print!("{index} ");
    }

    println!();
    
    let number_array = [10, 20, 30, 40, 50];

    for item in number_array {
        println!("item: {item}");
    }
}