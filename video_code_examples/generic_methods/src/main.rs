#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let point = Point { x: 13_u8, y: -4_i32};
    println!("{:?}", point);
    println!("{}", point.get_x());
}
