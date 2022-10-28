fn main() {
    let note: u8 = 105;

    if note > 100 {
        println!("The grade entered cannot be greater than 100!");
    } else if note >= 90 {
        println!("Grade A");
    } else if note >= 80 {
        println!("Grade B");
    } else if note >= 70 {
        println!("Grade C");
    } else if note >= 60 {
        println!("Grade D");
    } else {
        println!("You failed the course.");
    }
}