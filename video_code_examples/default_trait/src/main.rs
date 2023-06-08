#[derive(Debug)]
struct Car {
    name: String,
    range: u16
}

impl Default for Car {
    fn default() -> Self {
        Car {
            name: "togg".to_string(),
            range: 523_u16
        }
    }
}

fn main() {
    let togg = Car {
        range: 750_u16,
        ..Default::default()
    };
    println!("{:?}", togg);
}
