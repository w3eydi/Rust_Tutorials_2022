fn main() {
    let a = "eydi";

    {
        let b = "rust";
        println!("a: {a}, b:{b}");
    }
    
    //println!("b: {b}"); // This code gives an error.
    println!("a: {a}");

}