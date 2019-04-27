use crate::turing_machine::TuringMachine;
use crate::tape::{Tape, Tapeable};

pub enum ParseError {
}

/// Parse a String to create a simple DTM with one tape, expects the alphabet
/// to be used in the TM.
pub fn parse_simple_turing_machine<S: AsRef<str>, G: Tapeable>(string: S) -> Result<TuringMachine<G>, ParseError> {
    

    unimplemented!()
}
