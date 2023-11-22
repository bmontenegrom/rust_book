

fn main() {
    let s = String::from("hello");
    let largo = s.len();
    println!("{}", s);
    println!("el largo de {s} es {largo}");

    let s = String::from("hello world");
    let res = first_world(&s);
    println!("la primera palabra de {s} es: {res}");
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
