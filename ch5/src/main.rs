fn main() {
    println!("Hello, world!");

    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("rect is {rect:#?}");
    println!("The area of the rectangle is {} square pixels.", area(&rect));
    println!("The area of the rectangle is {} square pixels.", rect.area());
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let square = Rectangle::square(10);
    println!("The area of the square is {} square pixels.", square.area());
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// All functions defined within an impl block are called associated functions
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}
