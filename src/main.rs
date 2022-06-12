enum MachineState {
    EmptyState = 0,
    Space = 1,
    Character = 2,
    Number = 3,
    Error = -1
}

fn read_tape(tape: &str) {                     
    let state_array = [
                                                [MachineState::Error, MachineState::Space, MachineState::Character, MachineState::Number, MachineState::Error],
                                                [MachineState::Space, MachineState::Space, MachineState::Error, MachineState::Number, MachineState::Error],
                                                [MachineState::Character, MachineState::Error, MachineState::Error, MachineState::Error, MachineState::Error],
                                                [MachineState::Number, MachineState::Space, MachineState::Error, MachineState::Number, MachineState::Error],
                                                [MachineState::Error, MachineState::Error, MachineState::Error, MachineState::Error, MachineState::Error]
                                                ];

    let mut current_state = MachineState::EmptyState;
    let mut prev_state = MachineState::EmptyState;

    let mut cum_sum = 0;
    let mut new_str = String::new();
    

    for letter in tape.chars() {
        
    }
}


fn main() {
    let example_input = String::from("10 20 30 40 50 60");
}
