use std::ops::Deref;
use crate::letter::Letter;
use crate::replacement_rules::ReplacementRules;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Word<T>
where T: Letter
{
    container: Vec<T>,
}

pub enum WordError {
    IndexOutOfBoundsError { index: usize, len: usize },
}

// DISPLAY

impl<T> std::fmt::Display for Word<T>
where T: Letter + std::fmt::Display
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

impl<T> Word<T>
where T: Letter
{
    pub fn new() -> Self
    {
        Word {
            container: Vec::new(),
        }
    }

    pub fn push(&mut self, letter: T)
    {
        self.container.push(letter);
    }

    pub fn pop(&mut self) -> Option<T>
    {
        self.container.pop()
    }

    // Same as for insert goes for this function.
    pub fn remove(&mut self, index: usize) -> Result<T, WordError>
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
    pub fn insert(&mut self, index: usize, letter: T) -> Result<(), WordError> {
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

    //pub fn as_slice(&self) -> &[T] {
    //    self.container.as_slice()
    //}

    //pub fn as_mut_slice(&mut self) -> &mut [T] {
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

    //pub fn extend_from_slice(&mut self, other: &[T])
    //where T: Clone 
    //{
    //    self.container.extend_from_slice(other);
    //}

    //pub fn extend_from_within<R>(&mut self, src: R)
    //where 
    //    T: Clone,
    //    R: RangeBounds<usize>,
    //{
    //    self.container.extend_from_within(src);
    //}
}

// Deref

// Note to myself: Since we wrap a Vec<T>, and
// Vec<T> itself derefs to a slice [T], it is probably
// better to implement Deref for Word<T> with Target = Vec<T>,
// since this way, you get all the methods of Vec<T> AND [T]
// for Word<T> implicitly.
//
// Implementing Deref for Word<T> with Target = [T] would
// be appropriate if you want all methods on [T] implicitly
// for Word<T>, but you want to implement only a subset of the
// methods on Vec<T>, since now, you do not implicitly inherit
// all methods on Vec<T>.

//impl<T> Deref for Word<T>
//where T: Letter
//{
//    type Target = [T];
//    fn deref(&self) -> &Self::Target {
//        Deref::deref(&self.container)
//    }
//}

// Also: If you implement Deref here, you implicitly get the 
// clone method from Vec. This can be confusing if you want to 
// clone a word, but forget to derive Clone for it, since then,
// you get a Vec as the return type where you expect a Word.
impl<T> Deref for Word<T>
where T: Letter
{
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

// From Implementations to easily create a Word from other Types

impl<T> From<T> for Word<T>
where T: Letter
{
    fn from(letter: T) -> Self {
        Word {
            container: vec![letter],
        }
    }
}

impl<T> From<Vec<T>> for Word<T>
where T: Letter
{
    fn from(letters: Vec<T>) -> Self {
        Word {
            container: letters,
        }
    }
}

impl<T> From<&[T]> for Word<T>
where T: Letter + Clone
{
    fn from(letters: &[T]) -> Self {
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
impl<T> IntoIterator for Word<T>
where T: Letter
{
    type Item = T;
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
// In this case here, we only need to put T into std::slice::Iter,
// and not &'a T, since std::slice::Iter takes its inputs 'a and T 
// and then determines its type of iteration as &'a T, thus satisfying
// the trait bound on type IntoIter.
// This is in my opinion not obvious and can lead to confusing bugs 
// where the compiler complains about said trait bound.
// In short: If you implement IntoIterator, make sure how exactly the
// chosen specific iterator type for IntoIter determines its type 
// of iteration, and then reverse engineer its input so that its internal
// type of iteration matches the type you put in type Item.
impl<'a, T> IntoIterator for &'a Word<T>
where T: Letter
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.container.iter()
    }
}

// Iterator over mutable references
impl<'a, T> IntoIterator for &'a mut Word<T>
where T: Letter
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.container.iter_mut()
    }
}

// FROM_ITERATOR

// Create a word from an interator over Letters
impl<T> FromIterator<T> for Word<T>
where T: Letter
{
    fn from_iter<I>(iter: I) -> Self
    where I: IntoIterator<Item = T>
    {
        Word {
            container: iter.into_iter().collect(),
        }
    }
}

// Create a word from an iterator over Letter references
impl<'a, T> FromIterator<&'a T> for Word<T>
where T: Letter + Clone + 'a
{
    fn from_iter<I>(iter: I) -> Self
    where I: IntoIterator<Item = &'a T>
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
//impl<T> FromIterator<Word<T>> for Word<T>
//where T: Letter
//{
//    fn from_iter<I>(iter: I) -> Self
//    where I: IntoIterator<Item = Word<T>>
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
}
