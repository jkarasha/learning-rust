//use enums to demonstrate unreachable code
enum DoorState {
    Opened,
    Closed
}

enum DoorAction {
    Open,
    Close
}

fn take_action(current_state: DoorState, action: DoorAction) {
    match (current_state, action) {
        (DoorState::Opened, DoorAction::Close) => {
            println!("Door closing!")
        },
        (DoorState::Closed, DoorAction::Open) => {
            println!("Door opening!")
        },
        _ => unreachable!()
    }
}

fn main() {
    take_action(DoorState::Opened, DoorAction::Close);
    take_action(DoorState::Closed, DoorAction::Open);
    //Cannot open an already open door -> unreachable
    take_action(DoorState::Opened, DoorAction::Open);

}