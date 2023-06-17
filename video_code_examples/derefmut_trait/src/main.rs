use std::ops::{Deref, DerefMut};

struct Value(i32);

impl Deref for Value {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut value = Value(3);
    *value = 30;
    println!("dereferencing: {}", *value);
}