fn area_of_square(side: u64) -> (u64, u64) {
    let result = side * side;
    (result, side)
}

fn main() {
    let (area, side) = area_of_square(5);
    println!("area: {area}, side: {side}");
}
