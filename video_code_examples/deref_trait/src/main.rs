use std::ops::Deref;

struct Value(i32);

impl Deref for Value {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let value = Value(3);
    println!("dereferencing: {}", *value);    
}