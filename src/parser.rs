use crate::turing_machine::TuringMachine;
use crate::tape::{Tape, Tapeable};

use std::collections::HashMap;

pub enum ParseError {
    /// Lines need to be in the Implication Form, which means, that they are of
    /// the form premise -> effect
    NotImplicationForm,
    /// Encountered, when the String has an unaccepted type at a certain point,
    /// like a float, where an uint was expected
    InvalidType
}

/// Parse a String to create a simple DTM with one tape, expects the alphabet
/// to be used in the TM.
pub fn parse_simple_turing_machine<S: AsRef<str>, G: Tapeable + 'static>(src: S) -> Result<TuringMachine<G>, ParseError> {
    // Set to the default starting state and create an empty transition table
    let mut starting_state = 0;
    let mut transitions = HashMap::new();

    for l in src.as_ref().lines() {
        // Cut up the implication
        let clause: Vec<&str> = l.split("->").collect();
        if clause.len() != 2 { return Err(ParseError::NotImplicationForm); }

        let (cause, effect) = (clause[0].trim(), clause[1].trim());


    }

    Ok(TuringMachine::init_fully(Box::new(Tape::new()), transitions, starting_state))
}
