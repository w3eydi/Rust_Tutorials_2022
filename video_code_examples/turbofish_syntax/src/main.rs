struct Data<T> {
    value: T
}

fn main() {
    let data = Data::<i32> { value: 145 };
    let bool_data = Data::<bool> { value: true };
    println!("{}", data.value);
    println!("{}", bool_data.value);
}