//error[E0106]: missing lifetime specifier
//help: this function's return type contains a borrowed value, 
//but the signature does not say whether it is borrowed from `name` or `user`
fn ex(amt: i32, name: &str, user: &User) -> &str {
    unimplemented!()
}

struct User {
    userName: String,
}

fn main() {
    let test_user: User = User::new();
    println!("Hello, world!", ex(100, &String::from("Joe"), &test_user));
}
