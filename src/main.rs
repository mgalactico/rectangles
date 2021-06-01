#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //calculate area of rectangle
    fn area (&self) -> u32 {
        self.width * self.height
    }

    //compare rectangles
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //initialize a square rectangle
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };

    let rec4 = Rectangle::square(50);
    println!("rec4 is {:#?}", rec4);

    println!(
        "The area of rec1 is {} pixels wide.",
        rect1.area()
    );

    println!("Can rec1 hold rec 2? {}", rect1.can_hold(&rect2));
    println!("Can rec1 hold rec 2? {}", rect1.can_hold(&rect3));
}
