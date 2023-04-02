struct Vehicle {
    name: String,
    capacity: f32,
}

impl Vehicle {
    pub fn get_capacity(&self) -> f32 {
        self.capacity
    }

    // self, &self, mut self, &mut self
    fn add_charge(&mut self, value: f32) {
        self.capacity += value;
    }
}

fn main() {
    let mut togg = Vehicle {
        name: "Togg".to_string(),
        capacity: 88.5,
    };

    let cap_of_togg = togg.get_capacity();
    println!("{}", cap_of_togg);
    togg.add_charge(20.0);
    println!("{}", togg.get_capacity());
    println!("{}", togg.capacity);
}
