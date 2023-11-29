fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("the third elemento is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("the third element is {third}"),
        None => println!("there is no thir element."),
    }

    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];

    println!("antes de modificar");

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50
    }

    println!("despues de modificar");
    for i in &v {
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        match i {
            SpreadsheetCell::Int(x) => println!("un int: {x}"),
            SpreadsheetCell::Float(x) => println!("un float: {x}"),
            SpreadsheetCell::Text(x) => println!("una string: {x}"),
        }
    }

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 es {s2}"); //push_str usa &str y no toma posesión

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //s1 se movio a s3 no se puede usar más, el compilador transforma &String en &str
    println!("s3 es: {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s es :{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); //no se adueña de los parametros
    println!("usando format! s es :{s}");
    println!("s1 : {s1}, s2: {s2}, s3: {s3}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); //hash map se adueña de los valores que no implementan Copy, como String
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name}: {score} points");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); //insert dos veces sobre escribe
    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50); //si no está inserta
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); //devuelve &mut Value
        *count += 1;
    }
    println!("{:?}", map);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
