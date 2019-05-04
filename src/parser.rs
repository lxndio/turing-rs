use crate::turing_machine::{TuringMachine, State};
use crate::tape::{Direction, Tape, Tapeable};

use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

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

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    /// Lines need to be in the Implication Form, which means, that they are of
    /// the form premise -> effect
    NotImplicationForm,
    /// If the cause and or effect are not properly put into the brackets, this
    /// error will be thrown
    WrongBracketStructure,
    /// Too many arguments or too little in the cause or the effect clauses will
    /// result in this kind of error
    WrongNumberOfArguments,
    /// Encountered, when the String has an unaccepted type at a certain point,
    /// like a float, where an uint was expected
    InvalidType
}

/// Parse a String to create a simple DTM with one tape, expects the alphabet
/// to be used in the TM.
pub fn parse_simple_turing_machine<S: AsRef<str>, G>(src: S) -> Result<TuringMachine<G>, ParseError>
        where <G as FromStr>::Err: fmt::Debug, G: Tapeable + FromStr + 'static {
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
        let cause: Vec<&str> = cause.split(',').map(|x| x.trim()).collect();
        let effect: Vec<&str> = effect.split(',').map(|x| x.trim()).collect();

        if cause.len() != 2 || effect.len() != 3 {
            return Err(ParseError::WrongNumberOfArguments);
        }

        let cause_state: State = cause[0].parse().expect("Parsing Error in the starting state");
        let cause_char: Option<G> = match cause[1] {
            "None" => None,
            c => Some(c.parse().expect("Parsing error in the cause char"))
        };

        let effect_state: State = effect[0].parse().expect("Parsing Error in the resulting state");
        let effect_char: Option<G> = match effect[1] {
            "None" => None,
            c => Some(c.parse().expect("Parsing Error in the effect char"))
        };
        let movement = Direction::from_str(effect[2]).expect("Detected invalid direction");

        transitions.insert((cause_state, cause_char), (effect_state, effect_char, movement));
    }

    Ok(TuringMachine::init_fully(Box::new(Tape::new()), transitions, starting_state))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::turing_machine::Transitionable;

    #[test]
    fn test_simple_tm_parse() {
        let source = "() -> (1)
        (1, true) -> (1, false, Right)
        (1, false) -> (1, true, Right)
        (1, None) -> (1, None, Hold)".to_string();
        let tape = Tape::tape(vec![Some(true), Some(false), Some(true), Some(false), Some(true), Some(false)]);
        let mut tm = parse_simple_turing_machine(&source).expect("Could not parse turing machine");
        tm.insert_tape(Box::new(tape));

        while tm.step() {
            println!("Stepping TM");
        }

        println!("TM finished:");
        tm.print();

        assert_eq!(tm.tape().contents_trim_blanks(), vec![Some(false), Some(true), Some(false), Some(true), Some(false), Some(true)]);
    }
}
