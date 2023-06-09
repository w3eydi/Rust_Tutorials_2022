use std::fmt::Display;

struct Point {
    x: i32,
    y: i32
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let point = Point { x: 2, y: 11 };
    println!("Point: {}", point);
}