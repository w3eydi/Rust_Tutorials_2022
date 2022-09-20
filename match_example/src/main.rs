fn main() {
    let variable = 4;
    match variable {
        0 => println!("zero"),
        1 | 2 => {
            println!("one or two");
            println!("I don't know");
        }
        value @ 3..=10 => println!("3 <= {value} <= 10"),
        _ => println!("value: {variable}"),
    }
}