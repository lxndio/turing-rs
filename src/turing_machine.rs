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

    fn step(&mut self);
}

impl<V: Tapeable> Transitionable<V> for TuringMachine<V> {
    fn peek_transition(&self) -> (State, Option<V>, Direction) {
        *self.transitions.get(&(self.current_state, self.tape.read())).expect("Could not read from transition table")
    }

    fn step(&mut self) {

    }
}
