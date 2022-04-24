use std::ops::Deref;
use crate::letter::Letter;
use crate::replacement_rules::ReplacementRules;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Word<L>
where L: Letter
{
    container: Vec<L>,
}

pub enum WordError {
    IndexOutOfBoundsError { index: usize, len: usize },
}

// DISPLAY

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
where L: Letter
{
    pub fn new() -> Self
    {
        Word {
            container: Vec::new(),
        }
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

    //pub fn as_slice(&self) -> &[L] {
    //    self.container.as_slice()
    //}

    //pub fn as_mut_slice(&mut self) -> &mut [L] {
    //    self.container.as_mut_slice()
    //}

    //pub fn clear(&mut self) {
    //    self.container.clear();
    //}

    //pub fn len(&self) -> usize {
    //    self.container.len()
    //}

    //pub fn is_empty(&self) -> bool {
    //    self.container.is_empty()
    //}

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

    //pub fn extend_from_slice(&mut self, other: &[L])
    //where L: Clone 
    //{
    //    self.container.extend_from_slice(other);
    //}

    //pub fn extend_from_within<R>(&mut self, src: R)
    //where 
    //    L: Clone,
    //    R: RangeBounds<usize>,
    //{
    //    self.container.extend_from_within(src);
    //}
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

//impl<L> Deref for Word<L>
//where L: Letter
//{
//    type Target = [L];
//    fn deref(&self) -> &Self::Target {
//        Deref::deref(&self.container)
//    }
//}

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

impl<L> From<Vec<L>> for Word<L>
where L: Letter
{
    fn from(letters: Vec<L>) -> Self {
        Word {
            container: letters,
        }
    }
}

impl<L> From<&Vec<L>> for Word<L>
where L: Letter
{
    fn from(letters: &Vec<L>) -> Self {
        Word {
            container: letters.clone(),
        }
    }
}

impl<L> From<&[L]> for Word<L>
where L: Letter + Clone
{
    fn from(letters: &[L]) -> Self {
        let mut container = Vec::new();
        container.extend_from_slice(letters);
        Word {
            container,
        }
    }
}

// Create a Word of chars from a string slice.
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
}

// Create a word from an iterator over Letter references
impl<'a, L> FromIterator<&'a L> for Word<L>
where L: Letter + 'a
{
    fn from_iter<I>(iter: I) -> Self
    where I: IntoIterator<Item = &'a L>
    {
        Word {
            container: iter.into_iter().cloned().collect()
        }
    }
}

// Maybe we need am implementation of FromIterator to be able
// to collect an iterator over words into a word.
//
// We probably need to call flatten on top level
//impl<L> FromIterator<Word<L>> for Word<L>
//where L: Letter
//{
//    fn from_iter<I>(iter: I) -> Self
//    where I: IntoIterator<Item = Word<L>>
//    {
//        Word {
//            container: iter.into_iter().flatten().map(|word| word.container.as_slice()).collect(),
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

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
