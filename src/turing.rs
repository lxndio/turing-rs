use std::hash::Hash;
use std::collections::HashMap;
use crate::tape::{Direction, Tape, Value, X};

pub struct TuringMachine<V> {
    current_state: State,
    tape: Box<X<V>>,
    transitions: TransitionTable<V>,
}

pub type State = usize;
pub type TransitionTable<V> = HashMap<(State, Value<V>), (State, Value<V>, Direction)>;

pub trait Transitionable<V> {
    fn peek_transition(&self) -> (State, Value<V>, Direction);

    fn step(&mut self);
}

impl<V: Copy + Eq + Hash> Transitionable<V> for TuringMachine<V> {
    fn peek_transition(&self) -> (State, Value<V>, Direction) {
        *self.transitions.get(&(self.current_state, self.tape.read())).expect("Could not read from transition table")
    }

    fn step(&mut self) {

    }
}
