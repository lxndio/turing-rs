use std::fmt::{self, Debug, Display};
use std::hash::Hash;

pub trait Tapeable = Copy + Debug + Display + Eq + Hash;

pub struct Tape<V: Tapeable> {
    /// Tape positions from 0 to infinity
    positive_tape: Vec<Option<V>>,
    /// Tape positions from -1 to -infinity
    negative_tape: Vec<Option<V>>,
}

impl<V: Tapeable> Tape<V> {
    /// Create a new, empty tape
    pub fn new() -> Tape<V> {
        Tape {
            positive_tape: Vec::new(),
            negative_tape: Vec::new(),
        }
    }

    /// Create a Tape from the values in the slice
    pub fn tape(tape: Vec<Option<V>>) -> Tape<V> {
        Tape {
            positive_tape: tape,
            negative_tape: Vec::new(),
        }
    }

    pub fn read(&self, head_pos: isize) -> Option<V> {
        if head_pos >= 0 {
            if let Some(v) = self.positive_tape.get(head_pos as usize) { *v }
            else { None }
        }
        else {
            if let Some(v) = self.negative_tape.get(head_pos.abs() as usize - 1) { *v }
            else { None }
        }
    }

    pub fn write(&mut self, head_pos: isize, val: Option<V>) {
        self.fill_with_nones(head_pos);

        if head_pos >= 0 { self.positive_tape[head_pos as usize] = val; }
        else { self.negative_tape[head_pos.abs() as usize - 1] = val; }
    }

    /// Fill with None up to and including the index provided, in case the index
    /// is smaller than the lowest items pos or larger than the highest items pos.
    fn fill_with_nones(&mut self, until: isize) {
        if until >= 0 {
            let adj_hp = until as usize;
            if adj_hp  >= self.positive_tape.len() {
                self.positive_tape.resize_with(adj_hp + 1, || { None });
            }
        }
        else {
            let adj_hp = until.abs() as usize - 1;
            if adj_hp >= self.negative_tape.len() {
                self.negative_tape.resize_with(adj_hp + 1, || { None });
            }
        }
    }

    /// Get the tape contents as a slice. May contain leading or trailing blanks
    pub fn contents(&self) -> Vec<Option<V>> {
        self.negative_tape.clone().into_iter().rev().chain(self.positive_tape.clone().into_iter()).collect()
    }

    /// Like contents, but removes leading and trailing blanks. Blanks in the
    /// middle are accepted
    pub fn contents_trim_blanks(&self) -> Vec<Option<V>> {
        let neg = self.negative_tape.clone();
        let pos = self.positive_tape.clone();

        // Remove all Blanks from the start of the tape, but leave everything that may be trailing.
        let mut started = false;
        let mut res: Vec<Option<V>> = neg.into_iter().rev().chain(pos.into_iter())
            .filter(|x| {
                started |= x.is_some();
                x.is_some() || started
        }).collect();

        // Remove all blanks from the end of the tape, front has been dealt with already.
        while let Some(None) = res.last() { res.pop(); }
        res
    }
}

impl<V: Tapeable> Display for Tape<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "[ ")?;

        // Print the negative side (reversed, since the highest index contains
        // the leftmost value)
        for v in self.negative_tape.iter().rev() {
            if let Some(v) = v { write!(f, "{} ", v)?; }
            else { write!(f, "NONE")?; }
        }

        // Print the right side in the normal order
        for v in self.positive_tape.iter() {
            if let Some(v) = v { write!(f, "{} ", v)?; }
            else { write!(f, "NONE")?; }
        }

        write!(f, "]")?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tape_contents_trim_blanks() {
        let tape = Tape::tape(vec![None, None, Some(true), None, None, Some(true), None, None]);
        assert_eq!(tape.contents_trim_blanks(), vec![Some(true), None, None, Some(true)]);
    }

    #[test]
    fn tape_default_empty() {
        let tape: Tape<u8> = Tape::new();
        assert_eq!(tape.contents(), vec![]);
    }

    #[test]
    fn tape_contents() {
        let tape = Tape::tape(vec![Some(1010), None, None, Some(68), None]);
        assert_eq!(tape.contents(), vec![Some(1010), None, None, Some(68), None]);
    }

    #[test]
    fn tape_read_positive() {
        let tape = Tape::tape(vec![Some(-191), None, Some(1)]);
        assert_eq!(tape.read(2), Some(1));
    }

    #[test]
    fn tape_read_oob_neg() {
        let tape = Tape::tape(vec![Some("Hello"), Some(","), Some("there")]);
        assert_eq!(tape.read(-15701), None);
    }

    #[test]
    fn tape_read_oob_pos() {
        let tape = Tape::tape(vec![Some("Is"), None, Some("this"), None, Some("the"), None, Some("end?")]);
        assert_eq!(tape.read(4000), None);
    }

    #[test]
    fn tape_write_oob_neg() {
        let mut tape = Tape::new();
        tape.write(-42, Some(42));
        assert_eq!(tape.read(-42), Some(42));
    }

    #[test]
    fn tape_write_oob_pos() {
        let mut tape = Tape::new();
        tape.write(1337, Some("body once told me"));
        assert_eq!(tape.read(1337), Some("body once told me"));
    }
}
