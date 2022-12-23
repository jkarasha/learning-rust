// Cloning is an expensive operation and is not the best solution for larger objects
// Instead of cloning, you can use the & to share a reference to an object
struct Person {
    name: String,
}

fn congratulate(person: &Person) {
    println!("Congratulations, {} on your recent promotion!", person.name)
}

//Can you return a "borrowed" value from a function?
fn get_name() -> &str {
    let name = String::from("Joe");
    &name
}

fn main() {
    let person = Person {
        name: String::from("Joe"),
    };
    //
    congratulate(&person);
    // this will result in a borrowed here after move error
    // using .clone() on person doesn't work, unless you implment the method.
    // But there's a better way!
    println!("Can we still congratulate {} here?", person.name);

    // can we borrow from get_name?
    let my_name = get_name();

}
