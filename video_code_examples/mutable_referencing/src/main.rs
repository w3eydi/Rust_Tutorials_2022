fn main() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &s;
    println!("{}, {}", ref1, ref2);
    // variables ref1 and ref2 will not be used after this point
    let mut_ref3 = &mut s;
    println!("{}", mut_ref3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}