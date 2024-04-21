pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components : Vec<Box<dyn Draw>>, // <- components could be any type, is different than Generics
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    width:u32,
    height:u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        
    }
}

//Another type that implement draw button , but have different attributes. And could have another methods
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main(){
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}