fn main() {
    let name: &str = "eydi";
    let surname = String::from("gözeneli");
    let quick_definition = " university".to_string();
    let mut university = String::new();
    university.push_str("istanbul");
    university.push_str(quick_definition.as_str());
    university.push('!');

    println!("graduation: {}", university);
}