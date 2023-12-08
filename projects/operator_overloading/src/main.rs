use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        // can define  fn add(self, rhs: Point) -> Point
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Millimiters(u32);
struct Meters(u32);

impl Add<Meters> for Millimiters {
    //add meters to millimiters
    type Output = Millimiters;

    fn add(self, rhs: Meters) -> Millimiters {
        Millimiters(self.0 + (rhs.0 * 1000))
    }
}

fn main() {
    let p1 = Point { x: 1, y: 3 };
    let p2 = Point { x: 2, y: 0 };
    let p3 = p1 + p2;
    println!("p1 = {:?}", p1);
    println!("p2 = {:?}", p2);
    println!("p1 + p2 = {:?}", p3);
}
