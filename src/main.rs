#![allow(dead_code)]
#![feature(trait_alias)]

mod head;
mod lexicaliser;
mod parser;
mod tape;
mod turing_machine;

pub use head::*;
pub use lexicaliser::*;
pub use tape::*;
pub use turing_machine::*;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_turing_machine() {
        let tape = Tape::tape(vec![Some(true), Some(false), Some(true), Some(false), Some(true), Some(false)]);
        let mut tm = TuringMachine::new(tape);

        // Transitions for a simple turing machine that just inverts the input
        tm.add_transition((0, Some(true)), (0, Some(false), Direction::Right));
        tm.add_transition((0, Some(false)), (0, Some(true), Direction::Right));
        tm.add_transition((0, None), (0, None, Direction::Hold));

        while tm.step() {
            println!("Stepping TM");
        }

        println!("TM finished:");
        tm.print();

        assert_eq!(tm.tape().contents_trim_blanks(), vec![Some(false), Some(true), Some(false), Some(true), Some(false), Some(true)]);
    }
}
