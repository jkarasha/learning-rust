//Based on it's signature, last() call returns an option Either (Some) or (None)
//pub fn last_mut(&mut self) -> Option<&mut T>

fn main() {
    let nonempty_list = vec!['a', 'b', 'c'];
    println!("nonempty list's last element is : {:?}", nonempty_list.last());
    
    let empty_list: Vec<char> = vec![];
    println!("empty list's last element is : {:?}", empty_list.last());
}