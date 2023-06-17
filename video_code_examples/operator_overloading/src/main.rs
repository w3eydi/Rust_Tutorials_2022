use std::ops::Add;

#[derive(Debug)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Self { 0: self.0 + rhs.0, 1: self.1 + rhs.1 }
    }
}

fn main() {
    let p1 = Point(2, 3);
    let p2 = Point(4, 5);
    println!("{:?}", p1 + p2);
}