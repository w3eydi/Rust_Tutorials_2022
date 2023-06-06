use std::fmt::Debug;

struct Car {
    name: String,
    range: u16
}

impl Debug for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vehicle Information: name: {}, range: {}", self.name, self.range)
    }
}

fn main() {
    let togg = Car {
        name: "togg".to_string(),
        range: 523u16
    };
    println!("{:#?}", togg);
}
