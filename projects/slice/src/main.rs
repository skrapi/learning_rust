fn main() {
    let s = String::from("Hello world");
    
    println!("{}", first_world(&s));
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }   

    &s[..]
}
