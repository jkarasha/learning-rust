// Cloning is an expensive operation and is not the best solution for larger objects
// Instead of cloning, you can use the & to share a reference to an object
struct Person {
    name: String,
}

//Notice &Person? Now congratulate uses a "borrowed" instance parameter.
fn congratulate(person: &Person) {
    println!("Congratulations, {} on your recent promotion!", person.name)
}

// What happens when we borrow the person?
/* fn congratulate_borrowed(person: Person) {
    println!("Congratulations, {} on your recent promotion!", person.name)
} */

//Can you return a "borrowed" value from a function?
fn get_name() -> &str {
    let name = String::from("Joe");
    &name
}

fn main() {
    let person = Person {
        name: String::from("Joe"),
    };
    //congratulate, now borrows person instance from main.
    congratulate(&person);
    // Notice since main still owns the "person" instance, it can use it for it's own needs.
    println!("Can we still congratulate {} here?", person.name);
    // Now let's try borrowing the person?
    //congratulate_borrowed(person.clone());
    //Now if we try to use, it fails.Can we still use it.
    //println!("Notice that we can't congratulate {} on a borrowed instance?", person.name);
    congratulate(&person);
    // this will result in a borrowed here after move error
    // using .clone() on person doesn't work, unless you implment the method.
    // But there's a better way!
    //println!("Can we still congratulate {} here?", person.name);

    // can we borrow from get_name?
    let my_name = get_name();

}
