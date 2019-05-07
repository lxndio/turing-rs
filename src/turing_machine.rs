use std::collections::HashMap;
use crate::tape::{Tape, Tapeable};
use crate::head::{Direction, Head, TuringHead};

pub type State = usize;
pub type TransitionTable<V> = HashMap<(State, Option<V>), (State, Option<V>, Direction)>;

pub struct TuringMachine<V: Tapeable> {
    starting_state: State,
    current_state: State,
    tape: Tape<V>,
    head: Head,
    transitions: TransitionTable<V>,
}

pub trait Transitionable<V> {
    /// Reset the turing machine, so that it will start a calculation from the
    /// start, as if it was newly initialised again. Beware that the last state
    /// it was left in beforehand is not saved, so make sure to remember it
    /// somewhere else if it still matters.
    fn reset(&mut self);

    /// Check what the next transition will be, without actually performing it
    fn peek_transition(&self) -> (State, Option<V>, Direction);

    /// Perform the next transition. Return true, if the turing machine wants
    /// to continue running, false if it has stopped.
    fn step(&mut self) -> bool;
}

impl<V: Tapeable> TuringMachine<V> {
    /// Create a new turing machine with a tape inserted and empty transition function
    pub fn new(tape: Tape<V>) -> TuringMachine<V> {
        TuringMachine {
            starting_state: 0,
            current_state: 0,
            tape,
            head: Head::new(),
            transitions: HashMap::new()
        }
    }

    /// Create a new turing machine with a tape, empty transition table and a
    /// starting state that may differ from the default, which is 0
    pub fn with_starting_state(tape: Tape<V>, starting_state: State) -> TuringMachine<V> {
        TuringMachine {
            starting_state,
            current_state: starting_state,
            tape,
            head: Head::new(),
            transitions: HashMap::new()
        }
    }

    /// Initialise the turing machine fully. Needs the tape containing the input,
    /// the full transition table and the state the machine will start from.
    pub fn init_fully(tape: Tape<V>, transitions: TransitionTable<V>, starting_state: State) -> TuringMachine<V> {
        TuringMachine {
            starting_state,
            current_state: starting_state,
            tape,
            head: Head::new(),
            transitions
        }
    }

    /// Change the tape to be the one given as the argument. Keep in mind that
    /// the head position can change, since it is not bound to the turing
    /// machine, but to the tape itself.
    pub fn insert_tape(&mut self, tape: Tape<V>) {
        self.tape = tape;
    }

    /// Add a transition to the transition table. If there was already a transition
    /// registered to the cause, it will be replaced and returned
    pub fn add_transition(&mut self, cause: (State, Option<V>), effect: (State, Option<V>, Direction)) -> Option<(State, Option<V>, Direction)> {
        self.transitions.insert(cause, effect)
    }

    /// Print the state of the turing machine.
    pub fn print(&self) {
        println!("Turing Machine is in state: {}", self.current_state);
        println!("Transition Table:");
        for (k, v) in &self.transitions {
            println!("{:?} -> {:?}", k, v);
        }
        println!("Tape contents: {}", &self.tape);
    }

    pub fn tape(&self) -> &Tape<V> {
        &self.tape
    }
}

impl<V: Tapeable> Transitionable<V> for TuringMachine<V> {
    fn reset(&mut self) {
        self.current_state = self.starting_state;
    }

    fn peek_transition(&self) -> (State, Option<V>, Direction) {
        *self.transitions.get(&(self.current_state, self.head.read(&self.tape))).expect("Could not read from transition table")
    }

    /// Make the next step of the turing machine. Returns true, if it is still
    /// running. Returns false, if a holding state has been reached or an error
    /// has occured.
    fn step(&mut self) -> bool {
        let (new_state, value, dir) = self.peek_transition();

        // Check if a holding state has been reached
        if self.current_state == new_state && self.head.read(&self.tape) == value {
            println!("Reached holding state");
            return false;
        }

        // Change state and replace the tapes contents with the correct value
        self.current_state = new_state;
        self.head.write(&mut self.tape, value);
        self.head.mv(dir);

        true
    }
}
