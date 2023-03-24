#[derive(Debug)]
struct Person {
    name: String,
    surname: String,
    age: u8,
    work: String,
}

fn main() {
    let mut eydi = Person {
        name: "eydi".to_string(),
        surname: "gözeneli".to_string(),
        age: 31,
        work: "rust developer".to_string(),
    };

    println!("Hello, I'm {}. I'm a {} and I'm {} years old.",
        eydi.name,
        eydi.work,
        eydi.age);

    println!("{:?}", eydi);
    eydi.age = 32;
    eydi.work = "rustacean".to_string();
    println!("{eydi:?}");
}