#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32
}

fn area_of_rectangle(shape: Rectangle)-> u32{
    shape.length * shape.breadth
}

fn main() {
    let rect1 = Rectangle {
        length: 24,
        breadth: 10
    };

    println!("This is rectangle = {rect1:?}");
    let result = area_of_rectangle(rect1);

    print!("{}",result);
}
