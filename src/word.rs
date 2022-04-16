use std::ops::{RangeBounds, Deref};

use crate::letter::Letter;

pub struct Word<T>
where T: Letter
{
    container: Vec<T>,
}

impl<T> Word<T>
where T: Letter
{
    pub fn new() -> Self {
        Word {
            container: Vec::new(),
        }
    }

    pub fn push(&mut self, letter: T)
    where T: Clone
    {
        match self.pop() {
            Some(elem) => {
                let word = elem.combine(&letter);
                self.container.extend_from_slice(&word);
            },
            None => {
                let word: Word<T> = letter.into();
                self.container.extend_from_slice(&word);
            },
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.container.pop()
    }

    // Same as for insert goes for this function.
    pub fn remove(&mut self, index: usize) -> T {
        self.container.remove(index)
    }

    // Note: This function panics if index > len.
    // This function could be written to not panic 
    // by checking that index <= len first, and then
    // returning a Result instead.
    pub fn insert(&mut self, index: usize, letter: T) {
        self.container.insert(index, letter);
    }

    pub fn append(&mut self, word: &mut Self) {
        self.container.append(&mut word.container);
    }

    pub fn as_slice(&self) -> &[T] {
        self.container.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.container.as_mut_slice()
    }

    pub fn clear(&mut self) {
        self.container.clear();
    }

    pub fn len(&self) -> usize {
        self.container.len()
    }

    pub fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    pub fn split_off(&mut self, at: usize) -> Self {
        let split = self.container.split_off(at);
        Word {
            container: split,
        }
    }

    pub fn extend_from_slice(&mut self, other: &[T])
    where T: Clone 
    {
        self.container.extend_from_slice(other);
    }

    pub fn extend_from_within<R>(&mut self, src: R)
    where 
        T: Clone,
        R: RangeBounds<usize>,
    {
        self.container.extend_from_within(src);
    }

    pub fn extend_from_word(&mut self, other: Self)
    {
        for elem in other {
            self.container.push(elem);
        }
    }

    pub fn extend_from_word_ref(&mut self, other: &Self) 
    where T: Clone {
        for elem in other {
            self.container.push(elem.clone());
        }
    }

}

impl<T> Deref for Word<T>
where T: Letter
{
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        Deref::deref(&self.container)
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

// FromIterator
