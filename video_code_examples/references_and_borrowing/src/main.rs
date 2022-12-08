fn calculate_length(s: &String) -> usize {
    s.len()
}
fn main() {
    let s1 = "hello".to_owned();
    let len = calculate_length(&s1);
    println!("the length of '{}' is {}", s1, len);

    let ref_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}