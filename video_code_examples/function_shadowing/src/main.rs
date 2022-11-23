fn basket(count: String) -> String {
    println!("We have {} fruit basket(s)", count);
    count
}

fn main() {
    let number: String = "one".to_string();
    let number = basket(number);
    println!("fruit basket(s) amount: {number}");
}