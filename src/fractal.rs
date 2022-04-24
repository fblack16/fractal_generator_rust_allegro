use crate::letter::Letter;
use crate::word::Word;
use crate::replacement_rules::ReplacementRules;
use crate::alphabet::Alphabet;
use crate::semantics::Semantics;
use crate::dictionary::Dictionary;

use crate::semantics::Koch;

pub struct Fractal<L, S>
where
    L: Letter,
    S: Semantics,
{
        alphabet: Alphabet<L>,
        dictionary: Dictionary<L, S>,
        replacement_rules: ReplacementRules<Word<L>, Word<L>>,
        starting_word: Word<L>,
}

impl<L, S> Fractal<L, S>
where
    L: Letter,
    S: Semantics,
{
    pub fn new() -> Self
    {
        Fractal {
            alphabet: Alphabet::new(),
            dictionary: Dictionary::new(),
            replacement_rules: ReplacementRules::new(),
            starting_word: Word::new(),
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

        for &letter in &self.starting_word {
            let letter_as_word: Word<L> = letter.into();
            for (word, semantics) in &self.dictionary {
                if word == &letter_as_word {
                    semantics.execute();
                }
            }
        }
    }
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_replacement_rules() {
        let mut fractal: Fractal<char, Koch> = Fractal::new();
        fractal.alphabet.insert('F');
        fractal.alphabet.insert('+');
        fractal.alphabet.insert('-');

        fractal.dictionary.insert("F".into(), Koch::Forward);
        fractal.dictionary.insert("+".into(), Koch::TurnLeft);
        fractal.dictionary.insert("-".into(), Koch::TurnRight);

        fractal.replacement_rules.insert("F".into(), "F+F--F+F".into());
        fractal.starting_word = "F".into();

        fractal.apply_replacement_rules();
        assert_eq!(fractal.starting_word, "F+F--F+F".into());
    }

    #[test]
    fn test_apply_semantics() {
        let mut fractal: Fractal<char, Koch> = Fractal::new();
        fractal.alphabet.insert('F');
        fractal.alphabet.insert('+');
        fractal.alphabet.insert('-');

        fractal.dictionary.insert("F".into(), Koch::Forward);
        fractal.dictionary.insert("+".into(), Koch::TurnLeft);
        fractal.dictionary.insert("-".into(), Koch::TurnRight);

        fractal.replacement_rules.insert("F".into(), "F+F--F+F".into());

        fractal.starting_word = "F".into();
        fractal.apply_semantics();

        fractal.apply_replacement_rules();
        fractal.apply_semantics();
    }
}
