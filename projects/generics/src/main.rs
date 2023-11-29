use generics::{Tweet, Summary, NewsArticle, notify};

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<f64, f64> {
    fn distancia(&self) -> f64 {
       (&self.x.powi(2) + &self.y.powi(2)).sqrt() 
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("the largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("the largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = larges_generic(&char_list);
    println!("the largest element is {result}");

    let both_integer = Point { x: 5, y: 10 };
    println!("x is {} y is {}", both_integer.x, both_integer.y);

    let punto = Point { x: 4.0, y: 5 };

    println!("x es: {} e y: es {}", punto.x(), punto.y());

    let punto_float = Point{x:3.0, y:4.0};

    println!("la distancia al centro es: {}", punto_float.distancia());

    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let articulo = NewsArticle{
        headline: String::from("Penguins win Stanley Cup Championship!"),
        author:String::from("Iceburgh"),
        location: String::from("Pittsburgh, PA, USA"),
        content: String::from("The Pittsburgh Penguins once again are 
        the best hockey team in NHL."),
    };
    println!("New article available! {}", articulo.summarize());

    notify(&tweet);
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn larges_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
