use crate::{Tape, Tapeable};

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
            _other => None
        }
    }
}

/// A simple turing head, that can be used for the most basic of the operations
pub struct Head {
    /// The position the head is currently at. Starts at 0 by default
    pos: isize
}

impl Head {
    /// Create a new head at the default position
    pub fn new() -> Head {
        Head {
            pos: 0
        }
    }
}

impl TuringHead for Head {
    fn mv(&mut self, direction: Direction) {
        self.pos += direction as isize;
    }

    fn mv_left(&mut self) {
        self.mv(Direction::Left)
    }

    fn mv_right(&mut self) {
        self.mv(Direction::Right)
    }

    fn read<V: Tapeable>(&self, tape: &Tape<V>) -> Option<V> {
        tape.read(self.pos)
    }

    fn write<V: Tapeable>(&self, tape: &mut Tape<V>, val: Option<V>) {
        tape.write(self.pos, val);
    }

    fn pos(&self) -> isize { self.pos }
}

/// A turing head may only move one position left or right at a time to read on
/// or write to a tape.
pub trait TuringHead {
    /// Move the Head in any direction, or hold it by providing the direction.
    fn mv(&mut self, direction: Direction);

    /// Move the head left one position.
    fn mv_left(&mut self) ;

    /// Move the head right one position
    fn mv_right(&mut self);

    /// Read from the tape and return the value found on it.
    fn read<V: Tapeable>(&self, tape: &Tape<V>) -> Option<V>;

    /// Write the value to the tape at the current head position.
    fn write<V: Tapeable>(&self, tape: &mut Tape<V>, val: Option<V>);

    /// The current head position
    fn pos(&self) -> isize;
}
