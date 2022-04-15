use std::ops::RangeBounds;

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

    pub fn push(&mut self, letter: T) {
        self.container.push(letter);
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

}
