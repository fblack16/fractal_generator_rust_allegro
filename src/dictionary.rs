use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use crate::letter::Letter;
use crate::word::Word;
use crate::semantics::Semantics;

pub struct Dictionary<L, S>
where
    L: Letter,
    S: Semantics,
{
        container: HashMap<Word<L>, S>,
}

// IMPL

impl<L, S> Dictionary<L, S>
where
    L: Letter,
    S: Semantics,
{
    pub fn new() -> Self
    {
        Dictionary {
            container: HashMap::new(),
        }
    }
}

// DEREF

impl<L, S> Deref for Dictionary<L, S>
where
    L: Letter,
    S: Semantics,
{
    type Target = HashMap<Word<L>, S>;
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

impl<L, S> DerefMut for Dictionary<L, S>
where
    L: Letter,
    S: Semantics,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.container
    }
}

// ITERATOR

impl<'a, L, S> IntoIterator for &'a Dictionary<L, S>
where
    L: Letter,
    S: Semantics,
{
    type Item = (&'a Word<L>, &'a S);
    type IntoIter = std::collections::hash_map::Iter<'a, Word<L>, S>;
    fn into_iter(self) -> Self::IntoIter {
        self.container.iter()
    }
}

impl<'a, L, S> IntoIterator for &'a mut Dictionary<L, S>
where
    L: Letter,
    S: Semantics,
{
    type Item = (&'a Word<L>, &'a mut S);
    type IntoIter = std::collections::hash_map::IterMut<'a, Word<L>, S>;
    fn into_iter(self) -> Self::IntoIter {
        self.container.iter_mut()
    }
}
