use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use crate::semantics::Payload;
use crate::word::Word;

#[derive(Clone)]
pub struct Dictionary<T, P: Payload>(HashMap<Word<T>, DictionaryEntry<T, P>>);

#[derive(Clone)]
pub struct DictionaryEntry<T, P>
where
    P: Payload,
{
    replacement: Option<Word<T>>,
    semantics: Option<fn(word: &[T], payload: &mut P)>,
}

impl<T, P> DictionaryEntry<T, P>
where
    P: Payload,
{
    pub fn new() -> Self {
        DictionaryEntry { replacement: None, semantics: None }
    }

    pub fn with_replacement(mut self, replacement: Word<T>) -> Self {
        self.replacement = Some(replacement);
        self
    }

    pub fn with_semantics(mut self, semantics: fn(word: &[T], payload: &mut P)) -> Self {
        self.semantics = Some(semantics);
        self
    }

    pub fn clear_replacement(&mut self) {
        self.replacement = None;
    }

    pub fn clear_semantics(&mut self) {
        self.semantics = None;
    }

    pub fn add_replacement(&mut self, replacement: Word<T>) {
        self.replacement = Some(replacement);
    }

    pub fn add_semantics(&mut self, semantics: fn(word: &[T], payload: &mut P)) {
        self.semantics = Some(semantics);
    }

    pub fn semantics(&self) -> Option<fn(word: &[T], payload: &mut P)> {
        self.semantics
    }

    pub fn replacement(&self) -> Option<&Word<T>> {
        self.replacement.as_ref()
    }
}

impl<T, P> Display for DictionaryEntry<T, P>
where
    T: Display,
    P: Payload,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_replacement = match self.replacement {
            Some(ref word) => format!("{}", word),
            None => format!("None"),
        };

        let display_semantics = match self.semantics {
            Some(_) => format!("Some"),
            None => format!("None"),
        };

        write!(f, "DictionaryEntry {{ replacement: {}, semantics: {} }}", display_replacement, display_semantics)?;
        Ok(())
    }
}

impl<T, P> Display for Dictionary<T, P>
where
    T: Display + PartialEq + Eq + std::hash::Hash,
    P: Payload,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dictionary {{\n")?;
        for word in self.keys() {
            let display_entry = match self.get(word) {
                Some(entry) => format!("{}", entry),
                None => format!("None"),
            };
            write!(f, "\t{} -> {}\n", word, display_entry)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

// IMPL

impl<T, P> Dictionary<T, P>
where
    P: Payload,
{
    pub fn new() -> Self
    {
        Dictionary(HashMap::new())
    }

    pub fn with_words<I>(iter: I) -> Self
    where
        T: PartialEq + Eq + std::hash::Hash,
        I: IntoIterator<Item = Word<T>>,
    {
        let mut dictionary = Dictionary::new();
        for word in iter.into_iter() {
            dictionary.insert(word, DictionaryEntry::new());
        }
        dictionary
    }

    pub fn with_words_and_entries<I>(iter: I) -> Self
    where
        T: PartialEq + Eq + std::hash::Hash,
        I: IntoIterator<Item = (Word<T>, DictionaryEntry<T, P>)>,
    {
        let mut dictionary = Dictionary::new();
        for (word, entry) in iter.into_iter() {
            dictionary.insert(word, entry);
        }
        dictionary
    }
}

// DEREF

impl<T, P> Deref for Dictionary<T, P>
where
    P: Payload,
{
    type Target = HashMap<Word<T>, DictionaryEntry<T, P>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, P> DerefMut for Dictionary<T, P>
where
    P: Payload,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// FROM ITERATOR

impl<T, P> FromIterator<Word<T>> for Dictionary<T, P>
where
    T: PartialEq + Eq + std::hash::Hash,
    P: Payload,
{
    fn from_iter<I: IntoIterator<Item = Word<T>>>(iter: I) -> Self {
        Dictionary(iter.into_iter().map(|word| (word, DictionaryEntry::new())).collect())
    }
}

// Create a dictionary from an iterator over references to words
impl<'a, T, P> FromIterator<&'a Word<T>> for Dictionary<T, P>
where
    T: PartialEq + Eq + std::hash::Hash + Clone,
    P: Payload,
{
    fn from_iter<I: IntoIterator<Item = &'a Word<T>>>(iter: I) -> Self {
        Dictionary(
            iter.into_iter().map(
                |word| (word.clone(), DictionaryEntry::new())
            ).collect()
        )
    }
}

// Create a dictionary from an iterator over references to slices
impl<'a, T, P> FromIterator<&'a [T]> for Dictionary<T, P>
where
    T: PartialEq + Eq + std::hash::Hash + Clone,
    P: Payload,
{
    fn from_iter<I: IntoIterator<Item = &'a [T]>>(iter: I) -> Self {
        Dictionary(
            iter.into_iter().map(
                |word| (Word::from(word), DictionaryEntry::new())
            ).collect()
        )
    }
}
