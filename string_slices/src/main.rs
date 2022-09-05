fn main() {
    let name = String::from("eydi");
    let slice = &name[..2];

    println!("{}", slice);

    let hello = "say hi";
    let slice2 = &hello[1..5];

    println!("{}", slice2);
}