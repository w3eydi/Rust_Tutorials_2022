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

impl Point<u8, u8> {
    fn get_sum(&self) -> u8 {
        self.x + self.y
    }
}

fn main() {
    let point = Point { x: 13_u8, y: 4_u8};
    println!("{:?}", point);
    println!("{}", point.get_x());
    println!("{}", point.get_sum());
}