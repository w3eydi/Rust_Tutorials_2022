#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

fn main() {
    let p1 = Point { x: 1_u8, y: 45_u16 };
    println!("{:?}", p1);
    let p2_float = Point { x: 1.3, y: -4.2 };
    println!("{:?}", p2_float);
}