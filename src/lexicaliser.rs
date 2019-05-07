#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lex {
    /// Tuple containing some values that need to be interpreted
    /// according to the position in the string.
    Tuple(Vec<Option<String>>),
    /// Implication sign -> showing that something follows the introduction
    /// of the left cause.
    Implication
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LexError {
    UnexpectedToken(char),
    /// Thrown, when the lexicaliser reaches the end of the input, but something
    /// has not been finished up properly, resulting in the automaton finishing
    /// up in an unexpected state.
    InvalidHoldState(State)
}

/// State in which the lexicalisation automaton is currently in. Failure states
/// do not exist, since return of an error is immediate. This means, that multiple
/// errors cannot be detected.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    // Blank state, no information is currently being read.
    Blank,
    // Reading a tuple/a tuple element.
    InsideTuple,
    // Detected the start of a implication sign. The next character read must
    // always be a >, otherwise it will return an error
    ImplicationStart
}

pub fn lexicalise<S: AsRef<str>>(s: S) -> Result<Vec<Lex>, LexError> {
    // Starting state is blank, since we do not know, what to expect.
    let mut state = State::Blank;
    let mut res = Vec::new();
    let mut current_tuple: Vec<Option<String>> = Vec::new();
    let mut current_value = String::new();

    for c in s.as_ref().chars() {
        state = match state {
            State::Blank => handle_blank(c)?,
            State::InsideTuple => handle_inside_tuple(c, &mut res, &mut current_tuple, &mut current_value)?,
            State::ImplicationStart => handle_implication_start(c, &mut res)?
        };
    }

    // Check if the finishing state is correct
    if state != State::Blank {
        Err(LexError::InvalidHoldState(state))
    }
    else {
        Ok(res)
    }
}

fn handle_blank(c: char) -> Result<State, LexError> {
    match c {
        ' ' | '\n' => Ok(State::Blank),
        '(' => Ok(State::InsideTuple),
        '-' => Ok(State::ImplicationStart),
        other => Err(LexError::UnexpectedToken(other))
    }
}

fn handle_inside_tuple(c: char, res: &mut Vec<Lex>, t: &mut Vec<Option<String>>, v: &mut String) -> Result<State, LexError> {
    // Check if we are at the end of a tuple and add the finished lexicalised
    // element to the result vector if so
    if c == ')' {
        // Push the last tuple element, which might remain in the buffer
        if v.trim().len() != 0 {
            if v.trim() == "None" {
                t.push(None);
            }
            else {
                t.push(Some(v.trim().to_string()));
            }
        }

        // Push the tuple into the lexicalised buffer
        res.push(Lex::Tuple(t.clone()));
        t.clear();
        v.clear();
        Ok(State::Blank)
    }
    else if c == ',' {
        // Check if the value represents a blank tape character
        if v.trim() == "None" {
            t.push(None);
        }
        else {
            t.push(Some(v.trim().to_string()));
        }

        v.clear();
        Ok(State::InsideTuple)
    }
    else {
        v.push(c);
        Ok(State::InsideTuple)
    }
}

fn handle_implication_start(c: char, res: &mut Vec<Lex>) -> Result<State, LexError> {
    match c {
        '>' => {
            res.push(Lex::Implication);
            Ok(State::Blank)
        },
        other => Err(LexError::UnexpectedToken(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexicalisation() {
        let lexed = lexicalise("
            () -> (1)
            (1, true) -> (1, false, Right)
            (1, false) -> (1, true, Right)
            (1, None) -> (1, None, Hold)
            ").expect("Failed to lexicalise");

        assert_eq!(lexed, vec![
            Lex::Tuple(Vec::new()), Lex::Implication, Lex::Tuple(vec![Some("1".into())]),
            Lex::Tuple(vec![Some("1".into()), Some("true".into())]), Lex::Implication, Lex::Tuple(vec![Some("1".into()), Some("false".into()), Some("Right".into())]),
            Lex::Tuple(vec![Some("1".into()), Some("false".into())]), Lex::Implication, Lex::Tuple(vec![Some("1".into()), Some("true".into()), Some("Right".into())]),
            Lex::Tuple(vec![Some("1".into()), None]), Lex::Implication, Lex::Tuple(vec![Some("1".into()), None, Some("Hold".into())]),
        ]);
    }

    #[test]
    fn lex_unexpected_close_delim() {
        let lexed = lexicalise(" -> )");
        assert_eq!(lexed.err().unwrap(), LexError::UnexpectedToken(')'));
    }

    #[test]
    fn lex_unclosed_tuple_error() {
        let lexed = lexicalise("(a, b -> c");
        assert_eq!(lexed.err().unwrap(), LexError::InvalidHoldState(State::InsideTuple));
    }

    #[test]
    fn lex_unfinished_implication_error() {
        let lexed = lexicalise("(My) (life for) (the) (Horde) -> -> -");
        assert_eq!(lexed.err().unwrap(), LexError::InvalidHoldState(State::ImplicationStart));
    }

    #[test]
    fn lex_implication_destroyed() {
        let lexed = lexicalise("(Why are we still here?) -< (q, e, d)");
        assert_eq!(lexed.err().unwrap(), LexError::UnexpectedToken('<'));
    }
}
