use crate::turing_machine::{TuringMachine, State};
use crate::tape::{Direction, Tape, Tapeable};
use crate::lexicaliser::*;

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
    /// Too many arguments or too little in the cause or the effect clauses will
    /// result in this kind of error
    WrongNumberOfArguments,
    /// Encountered, when the String has an unaccepted type at a certain point,
    /// like a float, where an uint was expected
    InvalidType,
    /// If an implication is pointing towards nothing, this error is thrown
    ImplyingNothing,
    /// If the starting state is not left as the default, but is set to None,
    /// this will be returned, since the machine must have a valid starting state
    MustHaveStartingState,
    /// When a direction required, but not found.
    MissingDirection,
    /// When the lexicaliser stumbles upon an unexpected token, the semantics are
    /// not checked, but rather, this syntax error is thrown
    SyntaxError(LexError)
}

/// Parse a String to create a simple DTM with one tape, expects the alphabet
/// to be used in the TM.
pub fn parse_simple_turing_machine<S: AsRef<str>, G>(src: S) -> Result<TuringMachine<G>, ParseError>
        where <G as FromStr>::Err: fmt::Debug, G: Tapeable + FromStr + 'static {
    // Set to the default starting state and create an empty transition table
    let mut starting_state = 0;
    let mut transitions = HashMap::new();

    let lexed = match lexicalise(&src) {
        Ok(lex) => lex,
        Err(err) => return Err(ParseError::SyntaxError(err))
    };

    let mut i = lexed.iter();
    while let Some(cause) = i.next() {
        // Check that the next element is an implication sign
        if i.next() != Some(&Lex::Implication) {
            return Err(ParseError::NotImplicationForm);
        }
        // Check if the string suddenly ends
        let effect = match i.next() {
            Some(effect) => effect,
            None => return Err(ParseError::ImplyingNothing)
        };

        // Check if the cause and effect are actually tuples, and not some
        // nonsense
        let cause = match cause {
            Lex::Tuple(cause) => cause,
            _ => return Err(ParseError::NotImplicationForm)
        };
        let effect = match effect {
            Lex::Tuple(effect) => effect,
            _ => return Err(ParseError::NotImplicationForm)
        };

        // Check if it is the starting state in form () -> (q0)
        if cause.len() == 0 && effect.len() == 1 {
            if effect[0].is_none() {
                return Err(ParseError::MustHaveStartingState);
            }
            starting_state = match effect[0].clone().unwrap().parse() {
                Ok(ss) => ss,
                Err(_) => return Err(ParseError::InvalidType)
            };
        }
        else if cause.len() == 2 && effect.len() == 3 {
            let parse_state = |p: Option<String>| {
                let p = match p {
                    Some(p) => p,
                    None => return Err(ParseError::InvalidType)
                };

                match p.parse() {
                    Ok(p) => Ok(p),
                    Err(_) => Err(ParseError::InvalidType)
                }
            };

            let q: State = parse_state(cause[0].clone())?;
            let a = if let Some(a) = cause[1].clone() {
                match a.parse() {
                    Ok(a) => Some(a),
                    Err(_) => return Err(ParseError::InvalidType)
                }
            } else { None };

            let q_next = parse_state(effect[0].clone())?;
            let a_next = if let Some(a) = effect[1].clone() {
                match a.parse() {
                    Ok(a) => Some(a),
                    Err(_) => return Err(ParseError::InvalidType)
                }
            } else { None };
            let direction = match effect[2].clone() {
                Some(d) => d,
                None => return Err(ParseError::MissingDirection)
            };
            let direction = match Direction::from_str(direction) {
                Some(d) => d,
                None => return Err(ParseError::InvalidType)
            };

            transitions.insert((q, a), (q_next, a_next, direction));
        }
        else {
            return Err(ParseError::WrongNumberOfArguments);
        }
    }

    Ok(TuringMachine::init_fully(Box::new(Tape::new()), transitions, starting_state))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::turing_machine::Transitionable;

    #[test]
    fn test_simple_tm_parse() {
        let source = "
        () -> (1)
        (1, true) -> (1, false, Right)
        (1, false) -> (1, true, Right)
        (1, None) -> (1, None, Hold)
        ".to_string();
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
