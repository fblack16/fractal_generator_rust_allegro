use std::collections::HashSet;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use crate::letter::Letter;
use crate::word::Word;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dictionary<L>
where
    L: Letter,
{
    container: HashSet<Word<L>>,
}

impl<L> Display for Dictionary<L>
where
    L: Letter + Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dictionary {{ ")?;
        for word in &self.container {
            write!(f, "{} ", word)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

// IMPL

impl<L> Dictionary<L>
where
    L: Letter,
{
    pub fn new() -> Self
    {
        Dictionary {
            container: HashSet::new(),
        }
    }
}

// DEREF

impl<L> Deref for Dictionary<L>
where
    L: Letter,
{
    type Target = HashSet<Word<L>>;
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

impl<L> DerefMut for Dictionary<L>
where
    L: Letter,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.container
    }
}

// INTO ITERATOR

impl<L> IntoIterator for Dictionary<L>
where
    L: Letter,
{
    type Item = Word<L>;
    type IntoIter = std::collections::hash_set::IntoIter<Word<L>>;
    fn into_iter(self) -> Self::IntoIter {
        self.container.into_iter()
    }
}

impl<'a, L> IntoIterator for &'a Dictionary<L>
where
    L: Letter,
{
    type Item = &'a Word<L>;
    type IntoIter = std::collections::hash_set::Iter<'a, Word<L>>;
    fn into_iter(self) -> Self::IntoIter {
        self.container.iter()
    }
}

// FROM ITERATOR

impl<L> FromIterator<Word<L>> for Dictionary<L>
where
    L: Letter,
{
    fn from_iter<I: IntoIterator<Item = Word<L>>>(iter: I) -> Self {
        Dictionary {
            container: iter.into_iter().collect(),
        }
    }
}

// Create an Alphabet from an itertor over references to letters.
impl<'a, L> FromIterator<&'a Word<L>> for Dictionary<L>
where
    L: Letter,
{
    fn from_iter<I: IntoIterator<Item = &'a Word<L>>>(iter: I) -> Self {
        Dictionary {
            container: iter.into_iter().map(|word| word.clone()).collect(),
        }
    }
}
