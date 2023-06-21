trait Shape {
    fn area(&self) -> f64;
}
struct Square {
    side: f64
}

struct Circle {
    radius: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

fn dynamic_print_area(shape: &dyn Shape) {
    println!("area: {}", shape.area());
}

fn main() {
    let square = Square { side: 4. };
    let circle = Circle { radius: 5. };

    dynamic_print_area(&square);
    dynamic_print_area(&circle);
}