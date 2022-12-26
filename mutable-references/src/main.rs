#[derive(Debug)]

struct CarPool {
    passengers: Vec<String>
}

impl CarPool {
    //in order to update the object state, we have to make self mutable.
    fn pick_up(&mut self, name: String) {
        self.passengers.push(name);
    }
}
fn main() {

    let mut monday_car_pool = CarPool {
        passengers: vec![],
    };

    monday_car_pool.pick_up(String::from("Joe"));
    println!("Current carpool state {:?}", monday_car_pool);

    monday_car_pool.pick_up(String::from("Jessy"));
    println!("Current carpool state {:?}", monday_car_pool);
}