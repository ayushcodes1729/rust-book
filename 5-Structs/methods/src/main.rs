#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // We can also use the same name of the method same as one of the structs fields
        self.breadth * self.length
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.breadth >= rect.breadth && self.length >= rect.length
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 10,
        breadth: 24,
    };

    let rect2 = Rectangle {
        length: 2,
        breadth: 23,
    };

    let area = rect1.area();
    println!("{area}");

    if rect1.can_hold(&rect2) {
        println!("Rect 1 can hold rect 2")
    } else {
        println!("Rect 1 can't hold rect 2");
    }

    if rect2.can_hold(&rect1) {
        println!("Rect 2 can hold rect 1")
    } else {
        println!("Rect 2 can't hold rect 1");
    }
}
