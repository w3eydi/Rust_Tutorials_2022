fn main() {
    let value = 5;
    let result = if value == 5 { value + 10 } else { value - 5};
    let result = match value {
        0 => "zero",
        1 => "one",
        5 => "five",
        _ => "mismathed value",
    };

    let mut counter = 0;
    let result = loop {
        if counter == 10 {
            break "the number is ten"
        }

        counter += 1;
    };

    println!("result: {result}");
}