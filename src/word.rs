use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use crate::dictionary::Dictionary;
use crate::semantics::Payload;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Word<T>(Vec<T>);

//****************************************************************************
//
// Display Trait Implementation for Word<T>
//
//****************************************************************************

impl<T> std::fmt::Display for Word<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for elem in self.iter() {
            write!(f, "{}", elem)?;
        }
        Ok(())
    }
}

//****************************************************************************
//
// Implementation for Word<L>
//
//****************************************************************************

impl<T> Word<T>
{
    pub fn new() -> Self
    {
        Word(Vec::new())
    }

    pub fn first_subword(&self, valid_subwords: &[&Word<T>]) -> Option<&[T]>
    where
        T: PartialEq,
    {
        let mut longest_subword: &[T] = &[];
        for &word in valid_subwords {
            if !word.is_empty() && self[..].starts_with(word) {
                if longest_subword.starts_with(word) {
                    continue;
                } else {
                    longest_subword = &self[..word.len()];
                }
            }
        }
        if longest_subword.is_empty() {
            return None;
        }
        return Some(longest_subword);
    }

    fn first_subword_slice<'a>(word: &'a [T], valid_subwords: &[&'a Word<T>]) -> Option<&'a [T]>
    where
        T: PartialEq,
    {
        let mut longest_subword: &[T] = &[];
        for &subword in valid_subwords {
            if !subword.is_empty() && word.starts_with(subword) {
                if longest_subword.starts_with(subword) {
                    continue;
                } else {
                    longest_subword = &word[..subword.len()];
                }
            }
        }
        if longest_subword.is_empty() {
            return None;
        }
        return Some(longest_subword);
    }

    pub fn subwords<'a>(&'a self, valid_subwords: &[&'a Word<T>]) -> Vec<&'a [T]>
    where
        T: PartialEq,
    {
        let mut subwords = Vec::new();
        let mut start = 0;
        while let Some(word) = Word::first_subword_slice(&self[start..], valid_subwords) {
            subwords.push(word);
            start += word.len();
        }
        return subwords;
    }

    pub fn contains_word(&self, word: &[T]) -> bool
    where
        T: PartialEq + Eq,
    {
        if self.is_empty() || word.is_empty() {
            return false;
        }

        let mut is_contained = false;
        for (index, elem) in self.iter().enumerate() {
            if elem == &word[0] && &self[index..index+word.len()] == word {
                is_contained = true;
            }
        }

        return is_contained;
    }

    // Since for fractals, the new word can get really long really fast,
    // i don't see the benefit to try and reuse the memory currently owned 
    // by the word. Since the words get so long, a reallocation is probably necessary anyways.
    // Thus, we just return a brand new word.
    // This is probably going to get expensive really fast, at least if the letters of type T
    // need a lot of memory. On the other hand, if we use zero-sized types for T,
    // it is possible that this does not cost us a thing.
    pub fn apply_replacements<P: Payload>(&self, dictionary: &Dictionary<T, P>) -> Self
    where
        T: Copy + PartialEq + Eq + std::hash::Hash,
    {
        let valid_subwords: Vec<&Word<T>> = dictionary.keys().collect();
        let word = self.subwords(&valid_subwords).into_iter()
            .map(|word| {
                if let Some(entry) = dictionary.get(&Word::from(word)) {
                    match entry.replacement() {
                        Some(replacement) => replacement,
                        None => word,
                    }
                } else {
                    word
                }
            })
            .flatten()
            .collect();
        word
    }

    pub fn apply_semantics<P: Payload>(&self, dictionary: &Dictionary<T, P>, payload: &mut P)
    where
        T: Copy + PartialEq + Eq + std::hash::Hash,
    {
        let valid_subwords: Vec<&Word<T>> = dictionary.keys().collect();
        for word in self.subwords(&valid_subwords).into_iter() {
            if let Some(entry) = dictionary.get(&Word::from(word)) {
                match entry.semantics() {
                    Some(semantics) => semantics(&word, payload),
                    None => (),
                }
            }
        }
    }
}

//****************************************************************************
//
// Deref
//
//****************************************************************************

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

impl<T> Deref for Word<T>
{
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Word<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

//****************************************************************************
//
// FROM IMPLEMENTATION
//
//****************************************************************************

// Create a word from a single letter.
impl<T> From<T> for Word<T>
{
    fn from(letter: T) -> Self {
        Word(vec![letter])
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
impl<T> From<Vec<T>> for Word<T>
{
    fn from(letters: Vec<T>) -> Self {
        Word(letters)
    }
}

// Create a word from a reference to a vector of letters.
impl<T> From<&Vec<T>> for Word<T>
where
    T: Clone,
{
    fn from(letters: &Vec<T>) -> Self {
        Word(letters.clone())
    }
}

// Create a word from a slice of letters.
impl<T> From<&[T]> for Word<T>
where
    T: Clone,
{
    fn from(letters: &[T]) -> Self {
        Word(letters.to_vec())
    }
}

// Create a word of chars from a string slice.
impl From<&str> for Word<char>
{
    fn from(letters: &str) -> Self {
        Word(letters.chars().collect())
    }
}

//****************************************************************************
//
// INTO_ITERATOR IMPLEMENTATION
//
//****************************************************************************

// Iterator over values
impl<T> IntoIterator for Word<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
impl<'a, T> IntoIterator for &'a Word<T>
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// Iterator over mutable references
impl<'a, T> IntoIterator for &'a mut Word<T>
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

//****************************************************************************
//
// FROM_ITERATOR
//
//****************************************************************************

// Create a word from an iterator over letters
impl<T> FromIterator<T> for Word<T>
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Word(iter.into_iter().collect())
    }
}

// Create a word from an iterator over references to letters
impl<'a, T> FromIterator<&'a T> for Word<T>
where
    T: Clone,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a T>,
    {
        Word(iter.into_iter().map(|letter| letter.clone()).collect())
    }
}

//****************************************************************************
//
// TESTS
//
//****************************************************************************

#[cfg(test)]
mod tests {

    use super::*;
    use crate::dictionary::DictionaryEntry;

    #[test]
    fn test_display() {
        let numbers = vec![1, 2, 3, 4, 5];
        let chars = vec!['a', 'b', 'c', 'd', 'e'];

        let word_of_numbers: Word<usize> = numbers.into();
        let word_of_chars: Word<char> = chars.into();

        assert_eq!(format!("{}", word_of_numbers), "12345");
        assert_eq!(format!("{}", word_of_chars), "abcde");
    }

    #[test]
    fn test_first_subword_returns_none_for_empty_word() {
        let word = Word::new();
        let first = Word::from(1);
        let second = Word::from(2);
        let valid_subwords = vec![&first, &second];
        let first_subword = word.first_subword(&valid_subwords);
        assert!(first_subword.is_none());
    }

    #[test]
    fn test_first_subword_returns_none_if_valid_word_is_empty() {
        let word = Word::from(1);
        let first = Word::new();
        let valid_subwords = vec![&first];
        let first_subword = word.first_subword(&valid_subwords);
        assert!(first_subword.is_none());
    }

    #[test]
    fn test_first_subword_returns_none_if_valid_word_is_longer_than_word() {
        let word = Word::from(1);
        let first = Word::from(vec![1, 2]);
        let valid_subwords = vec![&first];
        let first_subword = word.first_subword(&valid_subwords);
        assert!(first_subword.is_none());
    }

    #[test]
    fn test_first_subword_for_word_of_usize() {
        let word: Word<i32> = Word::from(vec![1, 2, 3, 4, 5]);
        let valid_subword: Word<i32> = Word::from(vec![1, 2]);
        let valid_subwords = vec![&valid_subword];
        let first_subword = word.first_subword(&valid_subwords);
        assert!(first_subword.is_some());
        assert_eq!(first_subword.unwrap(), &valid_subword[..]);
    }

    #[test]
    fn test_subwords_on_empty_word() {
        let word: Word<usize> = Word::new();
        let first = Word::from(1);
        let second = Word::from(2);
        let third = Word::from(3);
        let valid_subwords = vec![&first, &second, &third];
        let subwords = word.subwords(&valid_subwords);
        assert!(subwords.is_empty());
    }

    #[test]
    fn test_subwords_with_empty_valid_word() {
        let numbers = vec![1, 2, 3];
        let word: Word<i32> = Word::from(&numbers);
        let first = Word::new();
        let valid_subwords = vec![&first];
        let subwords = word.subwords(&valid_subwords);
        assert!(subwords.is_empty());
    }

    #[test]
    fn test_subwords_on_word_of_usize() {
        let numbers = vec![1, 2, 3, 4, 5];
        let word: Word<i32> = Word::from(&numbers);
        let first = Word::from(&numbers[3..]);
        let second = Word::from(&numbers[..3]);
        let valid_subwords = vec![&first, &second];
        let subwords = word.subwords(&valid_subwords);
        assert_eq!(subwords, vec![&numbers[..3], &numbers[3..]]);
    }

    #[test]
    fn test_first_subword_context_free() {
        let word_of_numbers = Word::from_iter([1, 2, 3, 4, 5]);

        let first = Word::from_iter([1, 2]);
        let second = Word::from_iter([3]);
        let third = Word::from_iter([4, 5]);
        let fourth = Word::from_iter([6]);

        let dictionary: Dictionary<usize, ()> = Dictionary::with_words([first.clone(), second, third, fourth]);

        let valid_subwords: Vec<&Word<_>> = dictionary.keys().collect();
        let first_valid_word = word_of_numbers.first_subword(&valid_subwords);
        assert!(first_valid_word.is_some());
        assert_eq!(first_valid_word.unwrap(), &first[..]);
    }

    #[test]
    fn test_first_subword_context_dependent() {
        let word_of_numbers = Word::from_iter([1, 2, 3, 4, 5]);

        let first = Word::from_iter([1, 2]);
        let second = Word::from_iter([1, 2, 3]);

        let dictionary: Dictionary<usize, ()> = Dictionary::with_words([first, second.clone()]);

        let valid_subwords: Vec<&Word<_>> = dictionary.keys().collect();
        let first_valid_word = word_of_numbers.first_subword(&valid_subwords);
        assert!(first_valid_word.is_some());
        assert_eq!(first_valid_word.unwrap(), &second[..]);
    }

    #[test]
    fn test_subwords_context_free() {
        let word_of_numbers = Word::from_iter([1, 2, 3, 4, 5]);

        let first = Word::from_iter([1, 2]);
        let second = Word::from_iter([3]);
        let third = Word::from_iter([4, 5]);
        let fourth = Word::from_iter([6]);

        let dictionary: Dictionary<usize, ()> = Dictionary::with_words([first.clone(), second.clone(), third.clone(), fourth.clone()]);

        let valid_subwords: Vec<&Word<_>> = dictionary.keys().collect();
        let valid_words = word_of_numbers.subwords(&valid_subwords);
        assert_eq!(valid_words, vec![&first[..], &second[..], &third[..]]);
    }

    #[test]
    fn test_subwords_context_dependent() {
        let word_of_numbers = Word::from_iter([1, 2, 3, 4, 5]);

        let first = Word::from_iter([1, 2]);
        let second = Word::from_iter([1, 2, 3]);
        let third = Word::from_iter([4, 5]);
        let fourth = Word::from_iter([6]);

        let dictionary: Dictionary<usize, ()> = Dictionary::with_words([first.clone(), second.clone(), third.clone(), fourth.clone()]);

        let valid_subwords: Vec<&Word<_>> = dictionary.keys().collect();
        let valid_words = word_of_numbers.subwords(&valid_subwords);
        assert_eq!(valid_words, vec![&second[..], &third[..]]);
    }

    #[test]
    fn test_apply_replacements() {
        let initial: Word<char> = Word::from("f--f--f");
        let dictionary: Dictionary<char, ()> = Dictionary::with_words_and_entries([
            (Word::from("f"), DictionaryEntry::new().with_replacement(Word::from("f+f--f+f"))),
            (Word::from("+"), DictionaryEntry::new()),
            (Word::from("-"), DictionaryEntry::new()),
        ]);
        let result = initial.apply_replacements(&dictionary);
        assert_eq!(result, Word::from("f+f--f+f--f+f--f+f--f+f--f+f"));
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

        let word_of_functions: Word<fn(i32) -> i32> = Word::from(first);
        let dictionary: Dictionary<fn(i32) -> i32, ()> = Dictionary::with_words_and_entries([
            (Word::from(first), DictionaryEntry::new().with_replacement(Word::from_iter([second, third, first, third, second]))),
        ]);
        let result = word_of_functions.apply_replacements(&dictionary);
        assert_eq!(result, Word::from_iter([second, third, first, third, second]));
    }
}
