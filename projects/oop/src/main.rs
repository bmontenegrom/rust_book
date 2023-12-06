
use oop::{Button, Screen, Draw};


struct SelecBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelecBox {
    fn draw(&self) {
        //code
    }
}

fn main() {
    let screen = Screen{
        components: vec![
            Box::new(SelecBox{
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button{
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
        ],
    };
    screen.run();
}
