use crate::tape::Tape;

pub type State = usize;

pub struct TuringMachine<V: Copy = bool> {
    states: Vec<State>,
    tapes: Vec<Tape<V>>,
}


