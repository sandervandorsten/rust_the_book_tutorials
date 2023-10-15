// Allow Debug trait on the Struct
#[derive(Debug)]
struct Rectangle {
    width: u16,
    height: u16,
}

impl Rectangle {
    fn area(&self) -> u16 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u16) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let _scale = 2;
    let r1 = Rectangle {
        width: 30,
        height: 20,
    };
    let r2 = Rectangle {
        width: 40,
        height: 50,
    };
    dbg!(&r1);

    let r3 = Rectangle::square(4);
    let area: u16 = r1.area();
    println!("The area of r1='{r1:?}' is {area}");
    println!("r2 can hold r1? {}", r2.can_hold(&r1));
    println!("r1 can hold r2? {}", r1.can_hold(&r2));
}
