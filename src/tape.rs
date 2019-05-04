use std::fmt::{self, Debug, Display};
use std::hash::Hash;

pub trait Tapeable = Copy + Debug + Display + Eq + Hash;

pub struct Tape<V: Tapeable> {
    /// Tape positions from 0 to infinity
    positive_tape: Vec<Option<V>>,
    /// Tape positions from -1 to -infinity
    negative_tape: Vec<Option<V>>,
    /// The current head position
    head_position: isize
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Left = -1,
    Hold = 0,
    Right = 1
}

impl Direction {
    /// Parse a direction from a string. Returns None, if it is not a known
    /// direction.
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<Direction> {
        match s.as_ref().to_lowercase().as_ref() {
            "left" => Some(Direction::Left),
            "hold" => Some(Direction::Hold),
            "right" => Some(Direction::Right),
            other => None
        }
    }
}

pub trait SimpleTape<V>: Display {
    /// Move the Head in any direction, or hold it. Returns the Value on the
    /// Tape, which is at the new position.
    fn mv(&mut self, direction: Direction) -> Option<V>;

    /// Move the Head left and return the Value that is found there.
    fn mv_left(&mut self) -> Option<V>;

    /// Move the Head right and return the Value that is found there.
    fn mv_right(&mut self) -> Option<V>;

    /// Don't move the Head and read the Value that is right under it.
    fn read(&self) -> Option<V>;

    fn write(&mut self, val: Option<V>);

    /// Get the tape contents as a slice. May contain leading or trailing blanks
    fn contents(&self) -> Vec<Option<V>>;

    /// Like contents, but removes leading and trailing blanks. Blanks in the
    /// middle are accepted
    fn contents_trim_blanks(&self) -> Vec<Option<V>>;
}

impl<V: Tapeable> Tape<V> {
    /// Create a new, empty tape
    pub fn new() -> Tape<V> {
        Tape {
            positive_tape: Vec::new(),
            negative_tape: Vec::new(),
            head_position: 0
        }
    }

    /// Create a Tape from the values in the slice
    pub fn tape(tape: Vec<Option<V>>) -> Tape<V> {
        Tape {
            positive_tape: tape,
            negative_tape: Vec::new(),
            head_position: 0
        }
    }

    // Fill with None until the current head position, so that the infinite tape
    // rule will not be broken.
    fn fill_with_nones(&mut self) {
        if self.head_position >= 0 {
            let adj_hp = self.head_position as usize;
            if adj_hp  >= self.positive_tape.len() {
                self.positive_tape.resize_with(adj_hp + 1, || { None });
            }
        }
        else {
            let adj_hp = self.head_position.abs() as usize - 1;
            if adj_hp >= self.negative_tape.len() {
                self.negative_tape.resize_with(adj_hp + 1, || { None });
            }
        }
    }
}

impl<V: Tapeable> SimpleTape<V> for Tape<V> {
    fn mv(&mut self, direction: Direction) -> Option<V> {
        self.head_position += direction as isize;
        self.read()
    }

    fn mv_left(&mut self) -> Option<V> {
        self.mv(Direction::Left)
    }

    fn mv_right(&mut self) -> Option<V> {
        self.mv(Direction::Right)
    }

    fn read(&self) -> Option<V> {
        if self.head_position >= 0 {
            if let Some(v) = self.positive_tape.get(self.head_position as usize) { *v }
            else { None }
        }
        else {
            if let Some(v) = self.negative_tape.get(self.head_position.abs() as usize - 1) { *v }
            else { None }
        }
    }

    fn write(&mut self, val: Option<V>) {
        self.fill_with_nones();

        if self.head_position >= 0 { self.positive_tape[self.head_position as usize] = val; }
        else { self.negative_tape[self.head_position.abs() as usize - 1] = val; }
    }

    /// Get the tape contents as a slice. May contain leading or trailing blanks
    fn contents(&self) -> Vec<Option<V>> {
        self.negative_tape.clone().into_iter().rev().chain(self.positive_tape.clone().into_iter()).collect()
    }

    /// Like contents, but removes leading and trailing blanks. Blanks in the
    /// middle are accepted
    fn contents_trim_blanks(&self) -> Vec<Option<V>> {
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
    fn test_trim_blanks() {
        let tape = Tape::tape(vec![None, None, Some(true), None, None, Some(true), None, None]);
        assert_eq!(tape.contents_trim_blanks(), vec![Some(true), None, None, Some(true)]);
    }
}
