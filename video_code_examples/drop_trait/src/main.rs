struct Droppable {
    name: String
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("dropping: {}", self.name);
    }
}

fn main() {
    let a = Droppable { name: "a".to_string() };
    {
        let b = Droppable { name: "b".to_string() };
        println!("exiting the block!");
    }
    println!("after the exiting!");
    drop(a);
    println!("exiting the program!");
}