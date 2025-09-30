#[derive(Debug)]
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

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };

    let rect3 = Rectangle::square(3);



    println!("The area of the rectangle is {} square pixels.", rect1.area());

    println!("rect1 is {rect1:?}");

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("is rect3 a square? {}", rect3.is_square());
    println!(" is rect2 a square? {}", rect2.is_square());
}
