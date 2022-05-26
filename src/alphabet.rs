use std::collections::HashSet;
use std::fmt::Display;
use crate::letter::Letter;
use std::ops::{Deref, DerefMut};
use std::convert::From;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Alphabet<L>
where
    L: Letter,
{
    container: HashSet<L>,
}

impl<L> Alphabet<L>
where
    L: Letter,
{
    pub fn new() -> Self
    {
        Alphabet {
            container: HashSet::new(),
        }
    }
}

impl<L> Display for Alphabet<L>
where
    L: Letter + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Alphabet {{ ")?;
        for elem in &self.container {
            write!(f, "{} ", elem)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<L> Deref for Alphabet<L>
where
    L: Letter,
{
    type Target = HashSet<L>;
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

impl<L> DerefMut for Alphabet<L>
where
    L: Letter,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.container
    }
}

// Create an Alphabet from a single letter.
impl<L> From<L> for Alphabet<L>
where
    L: Letter,
{
    fn from(letter: L) -> Self {
        let mut container = HashSet::new();
        container.insert(letter);
        Alphabet {
            container,
        }
    }
}

impl From<&str> for Alphabet<char>
{
    fn from(letters: &str) -> Self {
        Alphabet {
            container: letters.chars().collect(),
        }
    }
}

// Iterator over owned values
impl<L> IntoIterator for Alphabet<L>
where
    L: Letter,
{
    type Item = L;
    type IntoIter = std::collections::hash_set::IntoIter<L>;
    fn into_iter(self) -> Self::IntoIter {
        self.container.into_iter()
    }
}

// Iterator over references
impl<'a, L> IntoIterator for &'a Alphabet<L>
where
    L: Letter,
{
    type Item = &'a L;
    type IntoIter = std::collections::hash_set::Iter<'a, L>;
    fn into_iter(self) -> Self::IntoIter {
        self.container.iter()
    }
}

// Create an Alphabet from an iterator over letters.
impl<L> FromIterator<L> for Alphabet<L>
where
    L: Letter,
{
    fn from_iter<I: IntoIterator<Item = L>>(iter: I) -> Self {
        Alphabet {
            container: iter.into_iter().collect(),
        }
    }
}

// Create an Alphabet from an itertor over references to letters.
impl<'a, L> FromIterator<&'a L> for Alphabet<L>
where
    L: Letter,
{
    fn from_iter<I: IntoIterator<Item = &'a L>>(iter: I) -> Self {
        Alphabet {
            container: iter.into_iter().map(|&letter| letter).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vector() {
        let chars = vec!['a', 'b', 'c'];
        let mut alphabet = Alphabet::from_iter(chars);
        assert!(alphabet.take(&'a').is_some());
        assert!(alphabet.take(&'b').is_some());
        assert!(alphabet.take(&'c').is_some());
    }

    #[test]
    fn test_from_slice() {
        let chars: &[char] = &['a', 'b', 'c'];
        let mut alphabet = Alphabet::from_iter(chars);
        assert!(alphabet.take(&'a').is_some());
        assert!(alphabet.take(&'b').is_some());
        assert!(alphabet.take(&'c').is_some());
    }

    #[test]
    fn test_from_string() {
        let text = "abc";
        let mut alphabet = Alphabet::from(text);
        assert!(alphabet.take(&'a').is_some());
        assert!(alphabet.take(&'b').is_some());
        assert!(alphabet.take(&'c').is_some());
    }
}
