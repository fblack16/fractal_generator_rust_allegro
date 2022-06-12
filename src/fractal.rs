use std::collections::HashSet;

use crate::word::Word;
use crate::semantics::Payload;
use crate::dictionary::{Dictionary, DictionaryEntry};

pub struct Fractal<T, P>
where
    P: Payload,
{
        alphabet: HashSet<T>, // the recognized letters
        dictionary: Dictionary<T, P>, // the recognized words, comprised of letters from the alphabet, with their possible replacements and semantics
        starting_word: Word<T>, // the starting word.
        word_stack: Vec<Word<T>>, // buffer for the iterated words
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
{
    pub fn new(payload: P) -> Self
    {
        Fractal {
            alphabet: HashSet::new(),
            dictionary: Dictionary::new(),
            starting_word: Word::new(),
            word_stack: Vec::new(),
            payload,
        }
    }

    // Will fail spectacularly if word_stack[0] is not initialized with the starting word.
    pub fn apply_replacements(&mut self, iterations: usize)
    where
        T: Copy + PartialEq + Eq + std::hash::Hash,
    {
        if iterations >= self.word_stack.len() {
            let additional_iterations = iterations - (self.word_stack.len()-1);
            for _ in 0..additional_iterations {
                self.word_stack.push(self.word_stack.last().unwrap().apply_replacements(&self.dictionary));
            }
        }
    }

    pub fn apply_semantics(&mut self)
    where
        T: Copy + PartialEq + Eq + std::hash::Hash,
    {
        if let Some(word) = self.word_stack.last() {
            word.apply_semantics(&self.dictionary, &mut self.payload);
        }
    }
}

pub fn Koch() -> Fractal<Koch, ()> {
    let alphabet = HashSet::from_iter([Koch::Forward, Koch::TurnLeft, Koch::TurnRight]);
    let payload = ();
    let dictionary = Dictionary::with_words_and_entries([
        (
            Word::from(Koch::Forward),
            DictionaryEntry::new()
                .with_replacement(
                    Word::from_iter(
                        [Koch::Forward, Koch::TurnLeft, Koch::Forward, Koch::TurnRight, Koch::TurnRight, Koch::Forward, Koch::TurnLeft, Koch::Forward]
                    )
                )
                .with_semantics(
                    |word, _| println!("{}", Word::<Koch>::from(word)),
                )
        ),
        (
            Word::from(Koch::TurnLeft),
            DictionaryEntry::new()
                .with_semantics(
                    |word, _| println!("{}", Word::<Koch>::from(word)),
                )
        ),
        (
            Word::from(Koch::TurnRight),
            DictionaryEntry::new()
                .with_semantics(
                    |word, _| println!("{}", Word::<Koch>::from(word)),
                )
        )
    ]);
    let starting_word = Word::from_iter(
        [Koch::Forward, Koch::TurnRight, Koch::TurnRight, Koch::Forward, Koch::TurnRight, Koch::TurnRight, Koch::Forward]
    );
    let word_stack = vec![starting_word.clone()];

    Fractal { 
        alphabet,
        dictionary,
        starting_word,
        word_stack,
        payload
    }
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_replacement_rules() {
        let mut koch = Koch();
        koch.apply_replacements(1);
        assert_eq!(
            koch.word_stack[1],
            Word::from_iter([
                Koch::Forward, Koch::TurnLeft, Koch::Forward, Koch::TurnRight, Koch::TurnRight, Koch::Forward, Koch::TurnLeft, Koch::Forward,
                Koch::TurnRight,
                Koch::TurnRight,
                Koch::Forward, Koch::TurnLeft, Koch::Forward, Koch::TurnRight, Koch::TurnRight, Koch::Forward, Koch::TurnLeft, Koch::Forward,
                Koch::TurnRight,
                Koch::TurnRight,
                Koch::Forward, Koch::TurnLeft, Koch::Forward, Koch::TurnRight, Koch::TurnRight, Koch::Forward, Koch::TurnLeft, Koch::Forward,
            ])
        )
    }

    #[test]
    fn test_apply_semantics() {
        let mut koch = Koch();
        koch.apply_semantics();
    }
}
