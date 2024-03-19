

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main(){

    let rect1 = Rectangle {
        width: 10,
        height: 10,
    };

    let rect2 = Rectangle::square(15);

    

    println!("The area of the rectangle1 is: {:?}", rect1.area());
    println!("The area of the rectangle2 is: {:?}", rect2.area());

    println!("Rect1 can hold Rect2: {:?}", rect1.can_hold(&rect2));
}