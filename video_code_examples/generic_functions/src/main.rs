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

fn generic_printer<T, U>(shape: T, size: U)
where
    T: Shape,
    U: std::fmt::Display
{
    print!("size: {} ", size);
    println!("area: {}", shape.area());
}

fn main() {
    let square = Square { side: 4. };
    let circle = Circle { radius: 5. };

    generic_printer(square, 4);
    generic_printer(circle, 5);
}