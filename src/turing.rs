use std::collections::HashMap;
use crate::tape::{Direction, Tape, Value, X};

pub struct TuringMachine<Value> {
    current_state: State,
    tape: Tape<Value>,
    transitions: TransitionTable<Value>,
}

pub type State = usize;
pub type TransitionTable<Value> = HashMap<(State, Value), (State, Value, Direction)>;

pub trait Transitionable<Value> {
    fn peek_transition(&self) -> &(State, Value, Direction);

    fn step(&mut self);
}

impl Transitionable<Value> for TuringMachine<Value> {
    fn peek_transition(&self) -> &(State, Value, Direction) {
        //self.transitions.get(&(self.current_state, self.tape.read())).expect("Could not read from transition table")
        unimplemented!()
    }

    fn step(&mut self) {

    }
}