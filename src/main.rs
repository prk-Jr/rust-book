mod rectangle;
use rectangle::Rectangle;
fn main() {
    println!("Hello, world!");

    let rect = Rectangle::new(50.0, 50.0);

    println!(
        "Rect is {:?} and the area is {:?} and is it square: {}",
        rect,
        rect.area(),
        match rect.is_square() {
            true => "yes",
            _ => "no",
        }
    );
}
