#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn create_square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    let area = rec.area();
    let square = Rectangle::create_square(5);

    println!("The area of {:#?} rectangle is equal to {area}", rec);
    println!("Our craeted square has these specifications: {:#?}", square);
}
