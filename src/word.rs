use std::ops::{Deref, Index, IndexMut, Range, RangeFrom, RangeFull, RangeTo, RangeInclusive, RangeToInclusive, RangeBounds};
use std::slice::SliceIndex;
use crate::dictionary::Dictionary;
use crate::letter::Letter;
use crate::replacement_rules::ReplacementRules;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Word<L>
where 
    L: Letter
{
    container: Vec<L>,
}

pub enum WordError {
    IndexOutOfBoundsError { index: usize, len: usize },
}

pub struct SubwordIter<'a, 'b, L>
where
    L: Letter,
{
    word: &'a [L],
    allowed_subwords: &'b Dictionary<L>,
}

impl<'a, 'b, L> Iterator for SubwordIter<'a, 'b, L>
where
    L: Letter,
{
    type Item = Word<L>;
    fn next(&mut self) -> Option<Self::Item> {
        let opt_subword = Word::from(self.word).get_first_valid_subword(self.allowed_subwords);
        if let Some(ref subword) = opt_subword {
            self.word = &self.word[subword.len()..];
        }
        return opt_subword;
    }
}

//****************************************************************************
//
// Index Trait Implementation for Word<L>
//
//****************************************************************************

// Index into a Word<L> using any type that implements the RangeBounds Trait.
// Notably, you can use all of Rust's range types.
impl<L, I> Index<I> for Word<L>
where
    L: Letter,
    I: SliceIndex<[L]>,
{
    type Output = <I as SliceIndex<[L]>>::Output;
    fn index(&self, index: I) -> &Self::Output {
        &self.container[index]
    }
}

//****************************************************************************
//
// IndexMut Trait Implementation for Word<L>
//
//****************************************************************************

impl<L, I> IndexMut<I> for Word<L>
where
    L: Letter,
    I: SliceIndex<[L]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.container[index]
    }
}

//****************************************************************************
//
// Display Trait Implementation for Word<L>
//
//****************************************************************************

impl<L> std::fmt::Display for Word<L>
where L: Letter + std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Word {{ ")?;
        for index in 0..self.len()-1 {
            write!(f, "{}, ", self.container[index])?;
        }
        write!(f, "{} }}", self.container[self.len()-1])?;
        Ok(())
    }
}

impl<L> Word<L>
where
    L: Letter
{
    pub fn new() -> Self
    {
        Word {
            container: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.container.len()
    }

    pub fn contains_letter(&self, letter: L) -> Option<usize> {
        for index in 0..self.len() {
            if self[index] == letter {
                return Some(index);
            }
        }
        None
    }

    pub fn first_letter(&self) -> L {
        self[0]
    }

    pub fn last_letter(&self) -> L {
        self[self.len()-1]
    }

    pub fn contains_word(&self, word: Word<L>) -> Option<Range<usize>> {
        let mut range = Range { start: 0, end: 1 };

        for index in 0..self.len() {
            if self[index] == word.first_letter() {
                let mut contained = true;

                for inner_index in 1..word.len() {
                    if index + inner_index >= self.len() {
                        contained = false;
                        break;
                    } else if word[inner_index] != self[index + inner_index] {
                        contained = false;
                        break;
                    }
                }

                if contained == true {
                    range.start = index;
                    range.end = index + word.len();
                    return Some(range);
                }
            }
        }
        return None;
    }

    pub fn push(&mut self, letter: L)
    {
        self.container.push(letter);
    }

    pub fn pop(&mut self) -> Option<L>
    {
        self.container.pop()
    }

    // Same as for insert goes for this function.
    pub fn remove(&mut self, index: usize) -> Result<L, WordError>
    {
        if index >= self.len() {
            return Err(WordError::IndexOutOfBoundsError { index, len: self.len() })
        }
        Ok(self.container.remove(index))
    }

    // Note: This function panics if index > len.
    // This function could be written to not panic 
    // by checking that index <= len first, and then
    // returning a Result instead.
    pub fn insert(&mut self, index: usize, letter: L) -> Result<(), WordError> {
        if index > self.len() {
            return Err(WordError::IndexOutOfBoundsError { index, len: self.len() })
        }
        self.container.insert(index, letter);
        Ok(())
    }

    pub fn append(&mut self, word: &mut Self) {
        self.container.append(&mut word.container);
    }

    // Note to myself: Since split_off allocates a new word, this method might be 
    // inefficient. Maybe this can be implemented more efficiently by first extending
    // the capacity of self, then inserting the individual elements.
    pub fn insert_word(&mut self, index: usize, word: &mut Self) -> Result<(), WordError>{
        if index > self.len() {
            return Err(WordError::IndexOutOfBoundsError { index, len: self.len() });
        }
        let mut end = self.split_off(index)?;
        self.append(word);
        self.append(&mut end);
        Ok(())
    }

    // Splits a word into two words at the given index.
    // Returns a Result with the newly allocated word containing the letters in the range [at, len),
    // or an Err if the given at index is out of bounds.
    // After the call, the original word will be left containing the elements [0, at),
    // with its previous capacity unchanged.
    pub fn split_off(&mut self, at: usize) -> Result<Self, WordError> {
        if at > self.len() {
            return Err(WordError::IndexOutOfBoundsError { index: at, len: self.len() });
        }
        let split = self.container.split_off(at);
        Ok(Word {
            container: split,
        })
    }

    // For this, we need that letters are copy or at least clone, and for clone,
    // we would need to do that explicitly in here.
    pub fn apply_replacement_rules(&mut self, rules: &ReplacementRules<Self, Self>) {
        self.container = self.container
            .iter() // iterate mutably over the letters in the container
            .map(|&letter| {
                let word: Self = letter.into();
                word
            }) // convert every letter to a word
            .map(|word| {
                if let Some(replacement) = rules.get(&word) {
                    replacement.clone()
                } else {
                    word
                }
            }) // replace the words that have a replacement
            .flatten()
            .collect(); // collect back into the container
    }

    // Note: This method assumes that there is no prefix to the first valid subword
    // that is invalid, that is, if there is a valid subword in the word,
    // but it is not right at the start, this method will return None.
    //fn get_first_valid_subword(&self, valid_subwords: &Dictionary<L>) -> Option<Self> {
    //    let mut highest_index = 0;

    //    for index in 1..=self.len() {
    //        let word = Word::from(&self[0..index]);
    //        if valid_subwords.contains(&word) {
    //            highest_index = index;
    //        }
    //    }

    //    if highest_index > 0 {
    //        return Some(Word::from(&self[0..highest_index]));
    //    }
    //    None
    //}

    fn get_first_valid_subword(&self, valid_subwords: &Dictionary<L>) -> Option<Self> {

        for index in (0..self.len()).rev() {
            let subword = Word::from(&self[..=index]);
            if valid_subwords.contains(&subword) {
                return Some(subword);
            }
        }

        return None;
    }

    fn get_valid_subwords(&self, valid_subwords: &Dictionary<L>) -> Vec<Self> {

        let mut contained_subwords = Vec::new();
        let mut next_start = 0;

        while let Some(word) = Word::from(&self[next_start..]).get_first_valid_subword(valid_subwords) {
            next_start += word.len();
            contained_subwords.push(word);
        }

        return contained_subwords;
    }

    pub fn subword_iter<'b>(&self, valid_subwords: &'b Dictionary<L>) -> SubwordIter<'_, 'b, L> {
        SubwordIter {
            word: &self,
            allowed_subwords: valid_subwords,
        }
    }
}

// Deref

// Note to myself: Since we wrap a Vec<L>, and
// Vec<L> itself derefs to a slice [L], it is probably
// better to implement Deref for Word<L> with Target = Vec<L>,
// since this way, you get all the methods of Vec<L> AND [L]
// for Word<L> implicitly.
//
// Implementing Deref for Word<L> with Target = [L] would
// be appropriate if you want all methods on [L] implicitly
// for Word<L>, but you want to implement only a subset of the
// methods on Vec<L>, since now, you do not implicitly inherit
// all methods on Vec<L>.

// Also: If you implement Deref here, you implicitly get the 
// clone method from Vec. This can be confusing if you want to 
// clone a word, but forget to derive Clone for it, since then,
// you get a Vec as the return type where you expect a Word.
impl<L> Deref for Word<L>
where L: Letter
{
    type Target = Vec<L>;
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

// From Implementations to easily create a Word from other Types

impl<L> From<L> for Word<L>
where L: Letter
{
    fn from(letter: L) -> Self {
        Word {
            container: vec![letter],
        }
    }
}

// Not sure if we need the next three implementations,
// as these should be taken care of by the FROM_ITERATOR
// implementations below.

// Note to myself: You technically do not need them, as you can
// create e.g. a Word of chars with Word::from_iter(chars) where chars is 
// some kind of collection that contains chars.
// BUT: This will not allow you to directly convert from the thing that implements 
// IntoIterator by calling into() on the thing.
// As explanation: A vector of characters can be converted to a Word of characters
// by calling Word::from_iter(vec), but if you try to just use vec.into() to convert 
// the vec to a word, or if you try to call Word::from(vec), these calls will fail.
// Thus, it is probably wise to implement From in parallel to FromIterator, to allow
// as many direct conversions as possible.

// Create a word from a vector of letters.
impl<L> From<Vec<L>> for Word<L>
where L: Letter
{
    fn from(letters: Vec<L>) -> Self {
        Word {
            container: letters,
        }
    }
}

// Create a word from a reference to a vector of letters.
impl<L> From<&Vec<L>> for Word<L>
where L: Letter
{
    fn from(letters: &Vec<L>) -> Self {
        Word {
            container: letters.clone(),
        }
    }
}

// Create a word from a slice of letters.
impl<L> From<&[L]> for Word<L>
where L: Letter
{
    fn from(letters: &[L]) -> Self {
        let mut container = Vec::new();
        container.extend_from_slice(letters);
        Word {
            container,
        }
    }
}

// Create a word of chars from a string slice.
impl From<&str> for Word<char>
{
    fn from(letters: &str) -> Self {
        let word = letters.chars().collect();
        word
    }
}

// Iterator

// Iterator over values
impl<L> IntoIterator for Word<L>
where L: Letter
{
    type Item = L;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.container.into_iter()
    }
}

// Iterator over references
//
// Important note to myself:
// There seems to be a trait bound on type IntoIter
// that is not mentioned in the Doc:
// type IntoIter: Iterator<Item = Self::Item>
// Read that as: The chosen specific iterator type for IntoIter
// needs to iterate over the same type as specified in type Item.
// In this case here, we only need to put L into std::slice::Iter,
// and not &'a L, since std::slice::Iter takes its inputs 'a and L 
// and then determines its type of iteration as &'a L, thus satisfying
// the trait bound on type IntoIter.
// This is in my opinion not obvious and can lead to confusing bugs 
// where the compiler complains about said trait bound.
// In short: If you implement IntoIterator, make sure how exactly the
// chosen specific iterator type for IntoIter determines its type 
// of iteration, and then reverse engineer its input so that its internal
// type of iteration matches the type you put in type Item.
impl<'a, L> IntoIterator for &'a Word<L>
where L: Letter
{
    type Item = &'a L;
    type IntoIter = std::slice::Iter<'a, L>;
    fn into_iter(self) -> Self::IntoIter {
        self.container.iter()
    }
}

// Iterator over mutable references
impl<'a, L> IntoIterator for &'a mut Word<L>
where L: Letter
{
    type Item = &'a mut L;
    type IntoIter = std::slice::IterMut<'a, L>;
    fn into_iter(self) -> Self::IntoIter {
        self.container.iter_mut()
    }
}

// FROM_ITERATOR

// Create a word from an interator over Letters
impl<L> FromIterator<L> for Word<L>
where L: Letter
{
    fn from_iter<I>(iter: I) -> Self
    where I: IntoIterator<Item = L>
    {
        Word {
            container: iter.into_iter().collect(),
        }
    }
} // Create a word from an iterator over Letter references
impl<'a, L> FromIterator<&'a L> for Word<L>
where L: Letter,
{
    fn from_iter<I>(iter: I) -> Self
    where I: IntoIterator<Item = &'a L>
    {
        Word {
            container: iter.into_iter().map(|&letter| letter).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_usize() {
        let word = Word::from_iter([1, 2, 3, 4, 5]);
        assert_eq!(word[0], 1);
        assert_eq!(word[1], 2);
        assert_eq!(word[2], 3);
        assert_eq!(word[3], 4);
        assert_eq!(word[4], 5);
    }

    #[test]
    #[should_panic]
    fn test_index_usize_panics() {
        let word = Word::from_iter([1, 2, 3, 4, 5]);
        let letter = word[5];
    }

    #[test]
    fn test_index_range() {
        let word = Word::from_iter([1, 2, 3, 4, 5]);
        assert_eq!(word[0..2], [1, 2]);
        assert_eq!(word[2..5], [3, 4, 5]);
    }

    #[test]
    #[should_panic]
    fn test_index_range_panics() {
        let word = Word::from_iter([1, 2, 3, 4, 5]);
        let slice_of_numbers = &word[4..6];
    }

    #[test]
    fn test_index_range_from() {
        let word = Word::from_iter([1, 2, 3, 4, 5]);
        assert_eq!(word[2..], [3, 4, 5]);
    }

    #[test]
    fn test_index_range_full() {
        let word = Word::from_iter([1, 2, 3, 4, 5]);
        assert_eq!(word[..], [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_push_and_pop_usize() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut word = Word::new();
        for &elem in &numbers {
            word.push(elem);
        }
        assert_eq!(word.container, numbers);
        for _ in 0..word.len() {
            word.pop();
        }
        assert_eq!(word.container, vec![]);
        word.pop();
        assert_eq!(word.container, vec![]);
    }

    #[test]
    fn test_push_and_pop_char() {
        let chars = vec!['a', 'b', 'c', 'd', 'e'];
        let mut word = Word::new();
        for &elem in &chars {
            word.push(elem);
        }
        assert_eq!(word.container, chars);
        for _ in 0..word.len() {
            word.pop();
        }
        assert_eq!(word.container, vec![]);
        word.pop();
        assert_eq!(word.container, vec![]);
    }

    #[test]
    fn test_push_and_pop_fn_i32_to_i32() {
        let functions: Vec<fn(i32) -> i32> = vec![|x| x+1, |x| 2*x, |x| x-3];
        let mut word = Word::new();
        for &elem in &functions {
            word.push(elem);
        }
        assert_eq!(word.container, functions);
        for _ in 0..word.len() {
            word.pop();
        }
        assert_eq!(word.container, vec![]);
        word.pop();
        assert_eq!(word.container, vec![]);
    }

    #[test]
    fn test_display() {
        let numbers = vec![1, 2, 3, 4, 5];
        let chars = vec!['a', 'b', 'c', 'd', 'e'];

        let word_of_numbers: Word<_> = numbers.into();
        let word_of_chars: Word<_> = chars.into();

        assert_eq!(format!("{}", word_of_numbers), "Word { 1, 2, 3, 4, 5 }");
        assert_eq!(format!("{}", word_of_chars), "Word { a, b, c, d, e }");
    }

    #[test]
    fn test_contains_word() {
        let numbers = vec![1, 2, 3, 4, 5];
        let word_of_numbers: Word<_> = numbers.into();

        let word_to_search = Word::from_iter([3, 4]);
        let optional = word_of_numbers.contains_word(word_to_search);
        assert!(optional.is_some());
        assert_eq!(optional.unwrap(), Range { start: 2, end: 4 });

        let word_to_search = Word::from_iter([6]);
        assert!(word_of_numbers.contains_word(word_to_search).is_none());

        let word_to_search = Word::from_iter([5, 6]);
        assert!(word_of_numbers.contains_word(word_to_search).is_none());
    }

    #[test]
    fn test_get_first_valid_subword_context_free() {
        let word_of_numbers = Word::from_iter([1, 2, 3, 4, 5]);

        let first = Word::from_iter([1, 2]);
        let second = Word::from_iter([3]);
        let third = Word::from_iter([4, 5]);
        let fourth = Word::from_iter([6]);

        let valid_subwords = Dictionary::from_iter([first, second, third, fourth]);

        let first_valid_word = word_of_numbers.get_first_valid_subword(&valid_subwords);
        assert!(first_valid_word.is_some());
        assert_eq!(first_valid_word.unwrap(), Word::from_iter([1, 2]));
    }

    #[test]
    fn test_get_first_valid_subword_context_dependent() {
        let word_of_numbers = Word::from_iter([1, 2, 3, 4, 5]);

        let first = Word::from_iter([1, 2]);
        let second = Word::from_iter([1, 2, 3]);

        let valid_subwords = Dictionary::from_iter([first, second]);

        let first_valid_word = word_of_numbers.get_first_valid_subword(&valid_subwords);
        assert!(first_valid_word.is_some());
        assert_eq!(first_valid_word.unwrap(), Word::from_iter([1, 2, 3]));
    }

    #[test]
    fn test_get_valid_subwords_context_free() {
        let word_of_numbers = Word::from_iter([1, 2, 3, 4, 5]);

        let first = Word::from_iter([1, 2]);
        let second = Word::from_iter([3]);
        let third = Word::from_iter([4, 5]);
        let fourth = Word::from_iter([6]);

        let valid_subwords = Dictionary::from_iter([first.clone(), second.clone(), third.clone(), fourth.clone()]);

        let valid_words = word_of_numbers.get_valid_subwords(&valid_subwords);
        assert_eq!(valid_words, vec![first, second, third]);
    }

    #[test]
    fn test_get_valid_subwords_context_dependent() {
        let word_of_numbers = Word::from_iter([1, 2, 3, 4, 5]);

        let first = Word::from_iter([1, 2]);
        let second = Word::from_iter([1, 2, 3]);
        let third = Word::from_iter([4, 5]);
        let fourth = Word::from_iter([6]);

        let valid_subwords = Dictionary::from_iter([first.clone(), second.clone(), third.clone(), fourth.clone()]);

        let valid_words = word_of_numbers.get_valid_subwords(&valid_subwords);
        assert_eq!(valid_words, vec![second, third]);
    }

    #[test]
    fn test_subword_iter_context_free() {
        let word_of_numbers = Word::from_iter([1, 2, 3, 4, 5]);

        let first = Word::from_iter([1, 2]);
        let second = Word::from_iter([3]);
        let third = Word::from_iter([4, 5]);
        let fourth = Word::from_iter([6]);

        let valid_subwords = Dictionary::from_iter([first.clone(), second.clone(), third.clone(), fourth.clone()]);
        let mut subword_iter = word_of_numbers.subword_iter(&valid_subwords);

        assert_eq!(subword_iter.next(), Some(first));
        assert_eq!(subword_iter.next(), Some(second));
        assert_eq!(subword_iter.next(), Some(third));
        assert_eq!(subword_iter.next(), None);
    }

    #[test]
    fn test_subword_iter_context_dependent() {
        let word_of_numbers = Word::from_iter([1, 2, 3, 4, 5]);

        let first = Word::from_iter([1, 2]);
        let second = Word::from_iter([1, 2, 3]);
        let third = Word::from_iter([4, 5]);
        let fourth = Word::from_iter([6]);

        let valid_subwords = Dictionary::from_iter([first.clone(), second.clone(), third.clone(), fourth.clone()]);
        let mut subword_iter = word_of_numbers.subword_iter(&valid_subwords);

        assert_eq!(subword_iter.next(), Some(second));
        assert_eq!(subword_iter.next(), Some(third));
        assert_eq!(subword_iter.next(), None);
    }

    #[test]
    fn test_apply_replacement_rules() {
        let mut word_of_chars: Word<char> = "f--f--f".into();
        let mut replacement_rules: ReplacementRules<Word<char>, Word<char>> = ReplacementRules::new();
        replacement_rules.insert("f".into(), "f+f--f+f".into());
        word_of_chars.apply_replacement_rules(&replacement_rules);
        assert_eq!(word_of_chars, "f+f--f+f--f+f--f+f--f+f--f+f".into());
    }

    #[test]
    fn test_apply_replacement_rules_functions() {
        // Note to myself: If you create function pointers (and closures) on the fly,
        // instead of putting them into first, second and third, this will not work,
        // since the assertion does not hold, because the call to apply replacement rules
        // will not detect a replacement for  |x| x+3, since two instances of |x| x+3 are 
        // NOT considered the same if they are created on the fly, e.g. they will have different
        // memory addresses, even though they describe the same function.
        // Thus, you need to save your closures / function pointers in a variable first, and then
        // use the variable subsequently (which is not a problem since function pointers are copy).
        // This ensures that you really get the same memory address everywhere and two "instances" 
        // are considered equal (since they literally are the same function).
        let first: fn(i32) -> i32 = |x| x+3;
        let second: fn(i32) -> i32 = |x| x+1;
        let third: fn(i32) -> i32 = |x| x*x;
        let functions: Vec<fn(i32) -> i32> = vec![first];
        let mut word_of_functions: Word<fn(i32) -> i32> = functions.into();
        let mut replacement_rules: ReplacementRules<Word<fn(i32) -> i32>, Word<fn(i32) -> i32>> = ReplacementRules::new();
        let to_replace: Vec<fn(i32) -> i32> = vec![first];
        let replacement: Vec<fn(i32) -> i32> = vec![second, third, first, third, second];
        replacement_rules.insert((&to_replace).into(), (&replacement).into());
        word_of_functions.apply_replacement_rules(&replacement_rules);
        assert_eq!(word_of_functions, (&replacement).into());
    }
}
