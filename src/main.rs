use std::ops::Mul;

#[derive(Debug)]
struct Rectangle<T> {
    height: T,
    width: T,
}
impl<T> Rectangle<T>
where
    T: PartialOrd<T> + Copy + Mul<T, Output = T>,
{
    fn new(height: T, width: T) -> Self {
        Self { height, width }
    }

    fn area(&self) -> T {
        self.height * self.width
    }

    fn is_square(&self) -> bool {
        self.height == self.width
    }
}

fn main() {
    println!("Hello, world!");

    let rect = Rectangle::new(50.0, 50.1);

    println!(
        "Rect is {:?} and area is {:?} and is it square {}",
        rect,
        rect.area(),
        rect.is_square()
    );
}
