struct Vehicle {
    name: String,
    capacity: f32,
}

impl Vehicle {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            capacity: 52.4
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let togg = Vehicle::new("Togg T10X v1");
    println!("vehicle: {}", togg.get_name());

    let togg_v2 = Vehicle::new("Togg T10X v2");
}