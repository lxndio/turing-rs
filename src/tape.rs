use std::hash::Hash;

pub struct Tape<Value> {
    /// Tape positions from 0 to infinity
    positive_tape: Vec<Value>,
    /// Tape positions from -1 to -infinity
    negative_tape: Vec<Value>,
    /// The current head position
    head_position: isize
}

pub type Value<V: Copy + Eq + Hash = bool> = Option<V>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Left = -1,
    Hold = 0,
    Right = 1
}

pub trait X<Value> {
    /// Move the Head in any direction, or hold it. Returns the Value on the
    /// Tape, which is at the new position.
    fn mv(&mut self, direction: Direction) -> Value;

    /// Move the Head left and return the Value that is found there.
    fn mv_left(&mut self) -> Value;

    /// Move the Head right and return the Value that is found there.
    fn mv_right(&mut self) -> Value;

    /// Don't move the Head and read the Value that is right under it.
    fn read(&self) -> Value;

    fn write(&mut self, val: Value);
}

impl<Value> Tape<Value> {
    /// Create a new, empty tape
    pub fn new() -> Tape<Value> {
        Tape {
            positive_tape: Vec::new(),
            negative_tape: Vec::new(),
            head_position: 0
        }
    }

    /// Create a Tape from the values in the slice
    pub fn tape(tape: Vec<Option<Value>>) -> Tape<Value> {
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

impl X<Value> for Tape<Value> {
    fn mv(&mut self, direction: Direction) -> Value {
        self.head_position += direction as isize;
        self.read()
    }

    fn mv_left(&mut self) -> Value {
        self.head_position -= 1;
        self.read()
    }

    fn mv_right(&mut self) -> Value {
        self.head_position += 1;
        self.read()
    }

    fn read(&self) -> Value {
        if self.head_position >= 0 { self.positive_tape[self.head_position as usize] }
        else { self.negative_tape[(self.head_position.abs()-1) as usize] }
    }

    fn write(&mut self, val: Value) {
        if self.head_position >= 0 { self.positive_tape[self.head_position as usize] = val }
        else { self.negative_tape[(self.head_position.abs()-1) as usize] = val }
    }
}
