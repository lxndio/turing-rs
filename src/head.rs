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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_from_str() {
        assert_eq!(Direction::from_str("Left"), Some(Direction::Left));
        assert_eq!(Direction::from_str("Hello, world"), None);
    }

    #[test]
    fn head_default_pos() {
        assert_eq!(Head::new().pos(), 0);
    }

    #[test]
    fn head_mv() {
        let mut head = Head::new();

        // Move to the right, left and then hold. The Head should be in exactly
        // the position from the start at the end
        head.mv(Direction::Left);
        head.mv(Direction::Right);
        head.mv(Direction::Hold);

        assert_eq!(head.pos(), 0);
    }

    #[test]
    fn head_mv_right() {
        let mut head = Head::new();
        head.mv_right();
        assert_eq!(head.pos(), 1);
    }

    #[test]
    fn head_mv_left() {
        let mut head = Head::new();
        head.mv_left();
        assert_eq!(head.pos(), -1);
    }

    #[test]
    fn head_read() {
        let mut head = Head::new();
        head.mv_right();
        let tape = Tape::tape(vec![Some(true), Some(false), None]);

        assert_eq!(head.read(&tape), Some(false));
    }

    #[test]
    fn head_write() {
        let mut head = Head::new();
        head.mv_left();
        let mut tape = Tape::tape(vec![Some(false), None, Some(true)]);

        head.write(&mut tape, Some(false));
        assert_eq!(head.read(&tape), Some(false));
    }
}
