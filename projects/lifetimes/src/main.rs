fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");


    let string3 = String::from("larga");
    {
        let string4 = String::from("xyz");
        let result_prueba = longest(string3.as_str(), &string4.as_str());
        println!("the longest string is {result_prueba}");
    };
    
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerp {
        part: first_sentence,
    };
    println!("imprimiento: {}", i.part)
    
}


fn longest<'a>(x: &'a str, y:&'a str) -> &'a str{
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

struct ImportantExcerp<'a>{
    part: &'a str,
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return  &s[0..i];
        }
    }
    &s[..]
 }