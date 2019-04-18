pub struct Tape<V = bool> {
    /// Tape positions from 0 to infinity
    positive_tape: Vec<Option<V>>,
    /// Tape positions from -1 to -infinity
    negative_tape: Vec<Option<V>>,
    /// The current head position
    head_position: usize
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Left = -1,
    Hold = 0,
    Right = 1
}

pub trait X {
    type Value;

    /// Move the Head in any direction, or hold it. Returns the Value on the
    /// Tape, which is at the new position.
    fn mv(&mut self, direction: Direction) -> Self::Value;

    /// Move the Head left and return the Value that is found there.
    fn mv_left(&mut self) -> Self::Value;

    /// Move the Head right and return the Value that is found there.
    fn mv_right(&mut self) -> Self::Value;

    /// Don't move the Head and read the Value that is right under it.
    fn read(&self) -> Self::Value;
}
