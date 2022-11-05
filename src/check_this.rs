use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum MyFirstLetter {
    Forward {length: f32},
    TurnLeft {angle: f32},
    TurnRight {angle: f32},
}

impl Display for MyFirstLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forward {length} => write!(f, "F({:?})", length),
            Self::TurnLeft {angle} => write!(f, "L({:?})", angle),
            Self::TurnRight {angle} => write!(f, "R({:?})", angle),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MySecondLetter {
    Forward {length: f32},
    TurnLeft {angle: f32},
    TurnRight {angle: f32},
}

impl Display for MySecondLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forward {length} => write!(f, "F({})", length),
            Self::TurnLeft {angle} => write!(f, "L({})", angle),
            Self::TurnRight {angle} => write!(f, "R({})", angle),
        }
    }
}

pub trait Letter: Copy + PartialEq {}

impl Letter for MyFirstLetter {}
impl Letter for MySecondLetter {}

#[derive(Clone, PartialEq)]
pub struct Word<L: Letter>(Vec<L>);

impl<L: Letter> Word<L> {
    pub fn new(letters: &[L]) -> Self {
        Self(letters.into())
    }
    // multiple things that are a problem here:
    // the need to create a new word in the comparison bothers me.
    // the function does not work as it should: we cannot bank on only
    // having production rules that have single letters as lhs.
    // we need to handle full words.
    // for that, we need to be able to extract all allowed subwords from the word
    // on which we apply the production rule
    // We need a method on words to append letters / other words.
    // we should also port other vector methods.
    // we should think about whether we want to consume the vector in the creation
    // of a word. in any case, if we stick with the ref, we should implement
    // from to also be able to consume words, as this might be more efficient
    // than copying every time.
    pub fn apply(&self, production_rule: &ProductionRule<L>) -> Self {
        let mut buffer = Vec::new();
        for &letter in self {
            if Word::new(&[letter]) == production_rule.lhs {
                //buffer.append(production_rule.rhs);
            } else {
                //buffer.push(letter);
            }
        }
        return Word::new(&buffer);

    }
}

impl<L: Letter + Display> Display for Word<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for letter in self {
            write!(f, "{}", letter)?;
        }
        Ok(())
    }
}

impl<L: Letter> IntoIterator for Word<L> {
    type Item = L;
    type IntoIter = std::vec::IntoIter<L>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, L: Letter> IntoIterator for &'a Word<L> {
    type Item = &'a L;
    type IntoIter = std::slice::Iter<'a, L>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, L: Letter> IntoIterator for &'a mut Word<L> {
    type Item = &'a mut L;
    type IntoIter = std::slice::IterMut<'a, L>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

pub trait WordMarker {}

impl<L: Letter> WordMarker for Word<L> {}

pub struct WordHolder {
    word: Box<dyn WordMarker>,
}

impl WordHolder {
    pub fn new(word: Box<dyn WordMarker>) -> Self {
        Self { word }
    }
    pub fn change_word(&mut self, word: Box<dyn WordMarker>) {
        self.word = word;
    }
}

pub struct ProductionRule<L: Letter> {
    lhs: Word<L>,
    rhs: Word<L>,
}

impl<L: Letter> ProductionRule<L> {
    pub fn new(lhs: &[L], rhs: &[L]) -> Self {
        Self {
            lhs: Word::new(lhs),
            rhs: Word::new(rhs),
        }
    }
}

impl<L: Letter> From<(&[L], &[L])> for ProductionRule<L> {
    fn from(rule: (&[L], &[L])) -> Self {
        Self {
            lhs: Word::new(rule.0),
            rhs: Word::new(rule.1),
        }
    }
}

impl<L: Letter> From<&(&[L], &[L])> for ProductionRule<L> {
    fn from(rule: &(&[L], &[L])) -> Self {
        Self {
            lhs: Word::new(rule.0),
            rhs: Word::new(rule.1),
        }
    }
}
