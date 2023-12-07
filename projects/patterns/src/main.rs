fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using blue as the background color");
        }
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        //enumerate returns a tuple, firs is the index then the value
        println!("{value} is at index {index}");
    }

    let point = (3, 5);
    print_coordinates(&point); //can use patterns in function parameters

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Go 50"),
        Some(y) => println!("Macthed, y = {y}"), //y shadows y from outside, match Some(5)
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the en : x = {:?}, y = {y}", x);

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"), //..= is an inclusive range
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; //can extract values from structs
    println!("destructuring point a = {a}, b = {b}");

    let Point { x, y } = p;
    println!("destructuring point x = {x}, y = {y}"); //can use only struct names

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither acis: ({x}, {y})"),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    let q = Message::Quit;
    let mov = Message::Move { x: 4, y: 5 };
    let w = Message::Write(String::from("hello"));
    let v = vec![msg, q, mov, w];

    for msg in v {
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destucture.")
            }
            Message::Move { x, y } => println!("Move the x dir {x}, in the y dir {y}"),
            Message::Write(text) => println!("Text message: {text}"),
            Message::ChangeColor(r, g, b) => {
                println!("Change color to red {r}, green {g}, and blue {b}")
            }
        }
    }

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            //if either setting_valure or new_setting_value is None, this branch executes
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        //usinf _s will bind s, and take ownership, can't use s later
        println!("found a string");
    }
    println!("{:?}", s);

    let origin = Point_3d { x: 0, y: 0, z: 0 };
    match origin {
        Point_3d { x, .. } => println!("x is {x}"),
    }

    match numbers {
        (first, .., last) => println!("Some numbers: {first}, {last}"), //.. must be unanbiguous .., second, .. wont work
    }

    let nums = vec![Some(4), Some(3), None];
    for num in nums {
        match num {
            Some(x) if x % 2 == 0 => println!("The number {x} is even"),
            Some(x) => println!("The number {x} is odd"),
            None => (),
        }
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 5 if y => println!("yes"),
        _ => println!("no"),
    }

    let msg = Message_2::Hello { id: 5 };

    match msg {
        Message_2::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message_2::Hello { id: 10..=12 } => println!("found an id in another range"),
        Message_2::Hello { id } => println!("Some other id: {id}"),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn foo(_: i32, y: i32) {
    //use _ to ignore parameter
    println!("This function only uses the y parameter: {y}");
}

struct Point_3d {
    x: i32,
    y: i32,
    z: i32,
}

enum Message_2 {
    Hello { id: i32 },
}
