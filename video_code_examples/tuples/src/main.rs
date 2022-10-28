fn main() {
    let tuple = (11, 2.97, 'z', (24, 'h'));
    println!("{}", tuple.3.0);
    let tuple2 = ('j', 3, -1.25);
    let (a, b, c) = tuple2;
    println!("a: {}, b: {}, c: {}", a, b, c);
}
