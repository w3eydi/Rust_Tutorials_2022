#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}

fn main() {
    let example_enum = Shape::Triangle(1.2, 2.1, 3.);
    println!("shape type: {:?}", example_enum);

    match example_enum {
        Shape::Circle(r) => println!("Circle's radius: {r}"),
        Shape::Rectangle(w, h) => println!("Rectangle dimensions: {w} x {h}"),
        // Shape::Triangle(a, b, c) => println!("Sides of triangle: {a}, {b}, {c}"),
        _ => println!("Unsupported shape!")
    }
}