use std::collections::HashMap;
use crate::tape::{Direction, Tapeable, SimpleTape};

pub type State = usize;
pub type TransitionTable<V> = HashMap<(State, Option<V>), (State, Option<V>, Direction)>;

pub struct TuringMachine<V: Tapeable> {
    current_state: State,
    tape: Box<SimpleTape<V>>,
    transitions: TransitionTable<V>,
}

pub trait Transitionable<V> {
    fn peek_transition(&self) -> (State, Option<V>, Direction);

    fn step(&mut self) -> bool;
}

impl<V: Tapeable> Transitionable<V> for TuringMachine<V> {
    fn peek_transition(&self) -> (State, Option<V>, Direction) {
        *self.transitions.get(&(self.current_state, self.tape.read())).expect("Could not read from transition table")
    }

    /// Make the next step of the turing machine. Returns true, if it is still
    /// running. Returns false, if a holding state has been reached or an error
    /// has occured.
    fn step(&mut self) -> bool {
        let (new_state, value, dir) = self.peek_transition();

        // Check if a holding state has been reached
        if self.current_state == new_state && self.tape.read() == value {
            println!("Reached holding state");
            return false;
        }

        // Change state and replace the tapes contents with the correct value
        self.current_state = new_state;
        self.tape.write(value);
        self.tape.mv(dir);

        true
    }
}
