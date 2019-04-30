# Specifications that a String must fulfill in order to be parseable.

### Deterministic turing machine with one tape and one transition function
The lines are read one by one and should adhere to the following format. The
meaning of the symbols is as follows:

- q - is the state, in which a character is read from the tape
- a - character that is being read, from the alphabet chosen, or None if it is a blank
- q' - state, in which the turing machine transitions after the procedure
- a' - character from the alphabet, or None for a blank, with which the character on the tape should be overwritten
- d - The direction, in which the head should move on the tape, options are left, right and hold

A line to create a function transition should be written as follows:

```(q, a) -> (q', a', d)```

The starting state is written as a transition with empty previous state and
nothing yet read, like this:

```() -> q0```

Since there will be nothing written yet and a move is not allowed before the
turing machine has started its work, a' and d are also left out.

Note that due to technical limitations, contrary to the theoretical turing
machine it is not possible to write any character onto the tape, but rather only
ones from the alphabet chosen, which should be the input alphabet combined with
the output alphabet and any intermediate characters needed.
TODO: Add functions to specify the allowed input and output characters.
It is expected from the users to check for the correct inputs and outputs
themselves.
