#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    let area = rec.area();

    println!("The area of {:#?} rectangle is equal to {area}", rec);
}
