fn main() {
    let raw = r##"He's going to speak: #"Hello, rust!"# "##;
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    println!("{raw}");
}