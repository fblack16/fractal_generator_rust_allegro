use std::collections::HashMap;
use std::hash::Hash;

use crate::{semantics::Payload, word_slice::Word};

pub struct Fractal<T, P>
where
    P: Payload,
    T: Clone + PartialEq + Eq + Hash,
{
        word_stack: Vec<Vec<T>>,
        replacements: HashMap<Vec<T>, Vec<T>>,
        semantics: HashMap<Vec<T>, fn(&mut P)>,
        payload: P,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Koch {
    Forward,
    TurnLeft,
    TurnRight,
}

impl std::fmt::Display for Koch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Koch::Forward => write!(f, "F")?,
            Koch::TurnLeft => write!(f, "+")?,
            Koch::TurnRight => write!(f, "-")?,
        }
        Ok(())
    }
}

impl<T, P> Fractal<T, P>
where
    P: Payload,
    T: Clone + PartialEq + Eq + Hash,
{
    pub fn new(payload: P) -> Self
    {
        Fractal {
            word_stack: vec![],
            replacements: HashMap::new(),
            semantics: HashMap::new(),
            payload,
        }
    }

    pub fn with_starting_word(&mut self, starting_word: Vec<T>)
    {
        self.word_stack[0] = starting_word;
    }

    pub fn apply_replacements(&mut self)
    {
        self.word_stack.push(
            self.word_stack
                .last()
                .unwrap()
                .apply_relacements(&self.replacements)
        );
    }

    pub fn iteration(&mut self, depth: usize) -> &[T]
    {
        while self.word_stack.len() <= depth {
            self.apply_replacements();
        }
        return self.word_stack[depth].as_slice();
    }

    pub fn apply_semantics(&mut self, depth: usize)
    {
        self.word_stack[depth].apply_semantics(&self.semantics, &mut self.payload);
    }
}

pub fn Koch(payload: String) -> Fractal<Koch, String>
{
    let word_stack = vec![
        vec![Koch::Forward, Koch::TurnRight, Koch::TurnRight, Koch::Forward, Koch::TurnRight, Koch::TurnRight, Koch::Forward],
    ];

    let mut replacements = HashMap::new();
    replacements.insert(
        vec![Koch::Forward],
        vec![Koch::Forward, Koch::TurnLeft, Koch::Forward, Koch::TurnRight, Koch::TurnRight, Koch::Forward, Koch::TurnLeft, Koch::Forward]
    );

    let mut semantics: HashMap<Vec<Koch>, fn(&mut String)> = HashMap::new();
    semantics.insert(
        vec![Koch::Forward],
        |s| {
            let to_push = format!("{}", Koch::Forward);
            s.push_str(&to_push);
        }
    );
    semantics.insert(
        vec![Koch::TurnLeft],
        |s| {
            let to_push = format!("{}", Koch::TurnLeft);
            s.push_str(&to_push);
        }
    );
    semantics.insert(
        vec![Koch::TurnRight],
        |s| {
            let to_push = format!("{}", Koch::TurnRight);
            s.push_str(&to_push);
        }
    );

    Fractal { 
        word_stack,
        replacements,
        semantics,
        payload,
    }
}

// TESTS

#[cfg(test)]
mod tests {

    mod apply_replacements {
        use crate::fractal::Koch;

        #[test]
        fn koch() {
            let mut koch = Koch(String::new());
            koch.apply_replacements();
            assert_eq!(
                koch.word_stack[1],
                vec![
                    Koch::Forward, Koch::TurnLeft, Koch::Forward, Koch::TurnRight, Koch::TurnRight, Koch::Forward, Koch::TurnLeft, Koch::Forward,
                    Koch::TurnRight,
                    Koch::TurnRight,
                    Koch::Forward, Koch::TurnLeft, Koch::Forward, Koch::TurnRight, Koch::TurnRight, Koch::Forward, Koch::TurnLeft, Koch::Forward,
                    Koch::TurnRight,
                    Koch::TurnRight,
                    Koch::Forward, Koch::TurnLeft, Koch::Forward, Koch::TurnRight, Koch::TurnRight, Koch::Forward, Koch::TurnLeft, Koch::Forward      
                ]
            );
        }
    }

    mod apply_semantics {
        use crate::fractal::Koch;

        #[test]
        fn koch() {
            let mut koch = Koch(String::new());
            koch.apply_semantics(0);
            assert_eq!(koch.payload, "F--F--F");
        }
    }
}
