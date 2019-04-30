use crate::turing_machine::TuringMachine;
use crate::tape::{Tape, Tapeable};

use std::collections::HashMap;

/// Helper function to get the contents that have been written in a string
/// starting with ( and ending with ). If something other than spaces is around
/// the brackets, returns None. If no brackets are found, returns None.
/// Otherwise returns contents of brackets, but without the brackets themselves
fn between_brackets(s: &str) -> Option<String> {
    let mut started = false;
    let mut finished = false;
    let mut failed = false;

    let r: String = s.trim().chars().filter(|x| {
        if !started && *x != '(' { failed = true; false }
        else if *x == '(' { started = true; false }
        else if finished { failed = true; false }
        else if *x == ')' { finished = true; false }
        else { true }
    }).collect();

    if !finished || !started || failed { None }
    else { Some(r) }
}

pub enum ParseError {
    /// Lines need to be in the Implication Form, which means, that they are of
    /// the form premise -> effect
    NotImplicationForm,
    /// If the cause and or effect are not properly put into the brackets, this
    /// error will be thrown
    WrongBracketStructure,
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
        let (cause, effect) = (between_brackets(&cause).expect("Error parsing brackets"), between_brackets(&effect).expect("Error parsing brackets"));

        // Check if the cause is empty, which would mark the starting state
        let spaceless: String = cause.chars().filter(|x| *x != ' ').collect();
        if spaceless == "" {
            starting_state = effect.parse().expect("Error parsing starting state");
            continue;
        }

        // Parse the transition and add it to the map
    }

    Ok(TuringMachine::init_fully(Box::new(Tape::new()), transitions, starting_state))
}
