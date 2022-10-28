fn main() {
    let num1 = 5;
    let num2 = 9;

    println!("num1: {:#010b}\nnum2: {:#010b}\n", num1, num2);
    println!("result: {:#010b}, decimal result: {}\n", 
    num1 & num2, num1 & num2);
}
