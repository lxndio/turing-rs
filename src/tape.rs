pub struct Tape<V: Copy = bool> {
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

    fn write(&mut self, val: Self::Value);
}

impl<V: Copy> Tape<V> {
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

impl<V: Copy> X for Tape<V> {
    type Value = Option<V>;

    fn mv(&mut self, direction: Direction) -> Option<V> {
        self.head_position += direction as isize;
        self.read()
    }

    fn mv_left(&mut self) -> Option<V> {
        self.head_position -= 1;
        self.read()
    }

    fn mv_right(&mut self) -> Option<V> {
        self.head_position += 1;
        self.read()
    }

    fn read(&self) -> Option<V> {
        if self.head_position >= 0 { self.positive_tape[self.head_position as usize] }
        else { self.negative_tape[(self.head_position.abs()-1) as usize] }
    }

    fn write(&mut self, val: Option<V>) {
        if self.head_position >= 0 { self.positive_tape[self.head_position as usize] = val }
        else { self.negative_tape[(self.head_position.abs()-1) as usize] = val }
    }
}
