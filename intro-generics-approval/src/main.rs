//define a generic struct
pub struct Approval<T> {
    item: T,
    approved: bool,
}

impl<T> Approval<T> {
    pub fn new(item: T) -> Approval<T> {
        Approval {
            item,
            approved: false,
        }
    }
    pub fn approve(&mut self) {
        self.approved = true;
    }
    pub fn replace(self, other_item: T) -> Approval<T> {
        Approval {
            item: other_item,
            approved: self.approved,
        }
    }
    pub fn approved_item(&self) -> Option<&T> {
        if self.approved {
            Some(&self.item)
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");
}
