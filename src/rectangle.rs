pub use std::ops::Mul;

#[derive(Debug)]
pub struct Rectangle<T> {
    pub height: T,
    pub width: T,
}
impl<T> Rectangle<T>
where
    T: PartialOrd<T> + Copy + Mul<T, Output = T>,
{
    pub fn new(height: T, width: T) -> Self {
        Self { height, width }
    }

    pub fn area(&self) -> T {
        self.height * self.width
    }

    pub fn is_square(&self) -> bool {
        self.height == self.width
    }
}
