//error handling using panic macro.
fn main() {
    //Will throw error/exception: thread 'main' panicked at 'Something's not right here', src/main.rs:3:5
    // panic!("Something's not right here");
    // You can also interpolate string values when calling panic.
    let age = "unknown";
    panic!("Something's not right here. We got the value '{}' for age!", age);
}
