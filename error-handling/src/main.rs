

fn main() {
    let v = vec![1, 2, 3];
    let index = 50;
    //expected error: thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 50', src/main.rs:7:53
    println!("The value at index {} is {}.", index, v[index]);
}