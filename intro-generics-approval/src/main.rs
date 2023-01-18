//define a generic struct
pub struct Approval<T> {
    item: T,
    approved: bool,
}

impl<T> Approval<T> {
    pub fn new(item: T) -> Approval<T> {
        unimplemented!()
    }
    pub fn approve(&mut self) {
        unimplemented!()
    }
    pub fn replace(self, other_item: T) -> Approval<T> {
        unimplemented!()
    }
    pub fn approved_item(&self) -> Option<&T> {
        unimplemented!()
    }
}

fn main() {
    println!("Hello, world!");
}
