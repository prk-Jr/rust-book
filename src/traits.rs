use std::default::Default;

pub trait MyNumber {
    fn swap(&mut self) -> ();
}

#[derive(Debug)]
pub struct Numbers {
    pub a: i32,
    pub b: i32,
}
impl MyNumber for Numbers {
    fn swap(&mut self) {
        let temp = self.a;
        self.a = self.b;
        self.b = temp;
    }
}

impl Numbers {
    pub fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }
}

impl Default for Numbers {
    fn default() -> Self {
        Self { a: 10, b: 5 }
    }
}
