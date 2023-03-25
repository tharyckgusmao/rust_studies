#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    // implementação de um metodo
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Retangle ,{:?}", rect1); // :? implementa a trait std::fmt::Debug

    let rect2 = Rectangle {
        length: 40,
        width: 10,
    };
    let rect3 = Rectangle {
        length: 45,
        width: 60,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
