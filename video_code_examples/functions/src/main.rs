fn area_of_rectangle(width: f64, height: f64) {
    println!("{width} x {height} rectangle's area: {}"
        , width * height);
}

fn main() {
    radius_ten_circle(15.0);
    area_of_rectangle(13.6, 4.3);
}

fn radius_ten_circle(radius: f64) {
    println!("Circumference of circle: {} with radius: {}"
        ,radius * 2.0 * 3.14, radius);
}