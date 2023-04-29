struct Point(i32, i32);

impl Point {
    fn get_x(&self) -> i32 {
        self.0
    }
}

fn main() {
    let point = Point(2, -4);
    println!("coordinate y: {}", point.1);
    println!("coordinate x: {}", point.get_x());
}
