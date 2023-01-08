fn main() {
    let list = vec![100,200, 300, 400, 500];
    print_first_two(&list);
    println!("List contents: {:?}", list);
    print_first_two(&list);
}

fn print_first_two(borrowed_list: &[i32]) {
    let first_two = &borrowed_list[0..2];
    println!("First two elements: {:?}", first_two);
}