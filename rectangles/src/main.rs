fn main() {
    // let width: u32 = 10;
    // let height: u32 = 20;

    // let rect1:(u32,u32) = (10,20);

    let rect2 = Rectangle{
        width: 30,
        height: 20,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width, height)
        // area(rect1)
        // area(&rect2)
        rect2.area()
    );

    println!("{:?},{:?}",rect2.width,rect2.height);

    println!("rect1 is {:#?}", rect2);

    let rect3 = Rectangle{
        width: 20,
        height: 50,
    };

    println!("can hold: {}", rect2.can_hold(&rect3));

    println!("square: {:?}", Rectangle::square(10));
}

// fn area (w: u32, h: u32) -> u32 {
//     w * h
// }
// fn area(dimensions: (u32,u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// #[derive(Debug)]
// struct Rectangle{
//     width: u32,
//     height: u32,
// }
// fn area(rect: &Rectangle) -> u32 {
//     rect.height * rect.width
// }

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height:size,
        }
    }
}