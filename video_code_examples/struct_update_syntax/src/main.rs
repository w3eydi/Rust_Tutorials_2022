#[derive(Debug)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    release_date: u16
}

fn main() {
    let phone_super = PhoneModel {
        company_name: "a company".to_string(),
        model_name: "super phone 2".to_string(),
        screen_size: 7.5,
        memory: 12,
        release_date: 2023
    };

    let phone_pro = PhoneModel {
        company_name: "b company".to_string(),
        model_name: "pro phone 1".to_string(),
        ..phone_super // struct update syntax
    };

    println!("{phone_super:?}");
    println!("{:?}", phone_pro);
}