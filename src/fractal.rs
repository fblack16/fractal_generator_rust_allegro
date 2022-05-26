use crate::letter::Letter;
use crate::semantics::{Semantics, Payload};
use crate::word::Word;
use crate::replacement_rules::ReplacementRules;
use crate::alphabet::Alphabet;
use crate::dictionary::Dictionary;

pub struct Fractal<L, P>
where
    L: Letter,
    P: Payload,
{
        alphabet: Alphabet<L>, // the recognized letters
        dictionary: Dictionary<L>, // the words that have semantics, the semantics may be context dependent
        replacement_rules: ReplacementRules<Word<L>, Word<L>>, // replacement rules for words
        semantics: Semantics<L, P>, // the semantics for the words
        starting_word: Word<L>, // the starting word.
        payload: P,
}

impl<L, P> Fractal<L, P>
where
    L: Letter,
    P: Payload,
{
    pub fn new(payload: P) -> Self
    {
        Fractal {
            alphabet: Alphabet::new(),
            dictionary: Dictionary::new(),
            replacement_rules: ReplacementRules::new(),
            semantics: Semantics::new(),
            starting_word: Word::new(),
            payload,
        }
    }

    pub fn apply_replacement_rules(&mut self)
    {
        self.starting_word.apply_replacement_rules(&self.replacement_rules);
    }

    pub fn apply_semantics(&self)
    {
        // Here lies the problem. We need to iterate over all possible subwords that have
        // semantics (given by the dictionary) that are contained in starting_word and execute
        // the semantics. This would work fine as long as the subwords are just words that contain 
        // exactly one letter, but this will not necessarily be the case, especially not for
        // context dependent Lindenmayer systems. For these, we would need to start searching for
        // the longest words that have semantics first, working our way down to words with only one 
        // letter.
        // -> We need to sort our Dictionary by the length of the words.
        //
        // For now, implement it only for words with a single letter (no context dependent
        // languages)

        for word in self.starting_word.subword_iter(&self.dictionary) {
            let action = self.semantics.get(&word).unwrap(); // todo error handling
            action(&word, &self.payload);
        }
    }
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_replacement_rules() {
        impl Payload for () {}
        let mut fractal: Fractal<char, ()> = Fractal::new(());
        fractal.alphabet.insert('F');
        fractal.alphabet.insert('+');
        fractal.alphabet.insert('-');

        fractal.dictionary.insert("F".into());
        fractal.dictionary.insert("+".into());
        fractal.dictionary.insert("-".into());

        fractal.replacement_rules.insert("F".into(), "F+F--F+F".into());
        fractal.semantics.insert("F".into(), |word, payload| {println!("Forward");});
        fractal.semantics.insert("+".into(), |word, payload| {println!("Turn Left");});
        fractal.semantics.insert("-".into(), |word, payload| {println!("Turn Right");});
        fractal.starting_word = "F".into();

        fractal.apply_replacement_rules();
        assert_eq!(fractal.starting_word, "F+F--F+F".into());
    }

    #[test]
    fn test_apply_semantics() {
        let mut fractal: Fractal<char, ()> = Fractal::new(());
        fractal.alphabet.insert('F');
        fractal.alphabet.insert('+');
        fractal.alphabet.insert('-');

        fractal.dictionary.insert("F".into());
        fractal.dictionary.insert("+".into());
        fractal.dictionary.insert("-".into());

        fractal.replacement_rules.insert("F".into(), "F+F--F+F".into());
        fractal.semantics.insert("F".into(), |word, payload| {println!("Forward");});
        fractal.semantics.insert("+".into(), |word, payload| {println!("Turn Left");});
        fractal.semantics.insert("-".into(), |word, payload| {println!("Turn Right");});

        fractal.starting_word = "F".into();
        fractal.apply_semantics();
    }
}
