fn main() {
    let value = 1;
    {
        println!("inner value: {value}");
        let value = "rust";
        println!("shadowing value: {value}");
    }
    println!("outer value: {value}");
    let value = true;
    println!("shadowing value: {value}");
}