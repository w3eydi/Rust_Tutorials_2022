trait AnimalSound {
    fn make_voice(&self) {
        println!("animal's voice!");
    }

    fn get_name(&self) -> String;
}

struct Dog {
    name: String,
    voice: String
}

struct Cat {
    name: String,
    voice: String
}

impl AnimalSound for Dog {
    fn make_voice(&self) {
        println!("Dog's voice: {}", self.voice);
    }
    fn get_name(&self) -> String {
        format!("Dog's name: {}", self.name)
    }
}
impl AnimalSound for Cat {
    fn get_name(&self) -> String {
        format!("Cat's name: {}", self.name)
    }
}

fn main() {
    let dog = Dog { name: "karabas".to_string(), voice: "Woof!".to_string() };
    println!("{}", dog.get_name());
    dog.make_voice();
    let cat = Cat { name: "boncuk".to_string(), voice: "Meow!".to_string() };
    println!("{}", cat.get_name());
    cat.make_voice();
}
