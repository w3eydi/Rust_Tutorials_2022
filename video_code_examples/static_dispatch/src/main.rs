trait Shape {
    fn area(&self) -> f64;
}

struct Square {
    side: f64
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

fn static_print_area(shape: &Square) {
    println!("area: {}", shape.area());
}

fn main() {
    let square = Square { side: 4. };
    static_print_area(&square);
}