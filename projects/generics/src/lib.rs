pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    /*fn summarize(&self) -> String {
        format!("{}, by {}({})", self.headline, self.author, self.location)
    }*/
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    // equivalente a: pub fn notify <T: Summary>(item &T)
    println!("Breaking news! {}", item.summarize());
}

/*
si tuviera dos elementos de tipos distintos:
    pub fn notify(item1: &impl Summary, item2: &impl Summary)

si tuviera dos elementos y quiero forzar que sena del mismo tipo
    pub fn notify<T: Summary>(item1: &T, item2: &T)

Para pedir que tengan mas de un Trait, se usa +:
    pub fun notify(item: &(impl Summary + Display)){}

    o

    pub fun notufy<T: Summary + Display>(item: &T){}

    En caso de ser muchos para mejor claridad usar where:

    fn some_function<T, U>(t: &T, u: &U) ->i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {}

    En ves de:
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u:&U ) -> i32 {}

*/

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> { //si quiero implementar una funcion para algo con algun trait en particular
    fn cmp_display(&self) {
        if self.x <= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The larges member is y = {}", self.y);
        }
    }
}
