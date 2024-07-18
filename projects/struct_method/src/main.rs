#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        if self.width > rect2.width && self.height > rect2.height {
            return true;
        }
        false
    }

    // Associated Functions
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 10,
        width: 50,
    };

    println!("{}", rect1.area());

    let rect2 = Rectangle {
        height: 20,
        width: 30,
    };

    println!("{}", rect1.can_hold(&rect2));

    let square = Rectangle::square(20);
    println!("{:#?}", &square);
}
