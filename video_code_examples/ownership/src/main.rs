fn calculate_length(s_string: String) -> (String, usize) {
    let length = s_string.len();
    (s_string, length)
}
fn main() {
    let s = "hello".to_owned();
    let (s, len) = calculate_length(s);
    println!("the length of '{}' is {}", s, len);
}
