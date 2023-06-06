#[derive(Debug, Clone)]
struct Car {
    name: String,
    range: u16
}

fn main() {
    let togg = Car {
        name: "togg".to_string(),
        range: 523_u16
    };
    println!("{:?}", togg);
    let togg2 = togg.clone();
    println!("{:?}", togg2);
}