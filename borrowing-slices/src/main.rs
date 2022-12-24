// slicing works very similiar to how it works in Python
// [begin..end], starts at "begin" and stops before "end"
// begin is inclusive, end is not

fn main() {
    let greeting = String::from("Hello, World!");
    //
    let string_slice = &greeting[2..4];
    println!("The slice from 2 to 4 is {}.", string_slice);
    // returns the first 4 chars   
    let string_slice = &greeting[0..4];
    println!("The first four chars in greeting are {}.", string_slice);
    let string_slice = &greeting[..4];
    println!("The slice without a starting index up to 4 is {}", string_slice);
    let string_slice = &greeting[2..];
    println!("The slice from 2 without an ending index is {}", string_slice);
    let string_slice = &greeting[..];
    println!("The slice of the entire string is {}", string_slice);

    // slicing is not limited to just strings, works for arrays and vectors too.
    let a = [0.0, 3.14, -7.839];
    let array_slice = &a[..2];
    // [{float}]` cannot be formatted with the default format. Use {:?}
    println!("The first two elements of the array are {:?}", array_slice);
    let array_slice = &a[1..];
    println!("The last two elements of the array are {:?}", array_slice);
    //
    // show using mutable vector
    let mut v = vec![10, 20, 30];
    v.push(40);
    let vector_slice = &v[1..2];
    println!("The second element of the vecctor is {:?}", vector_slice);
    let vector_slice = &v[..3];
    println!("The first three elements of the vecctor are {:?}", vector_slice);

}
