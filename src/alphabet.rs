use std::collections::HashSet;
use crate::letter::Letter;
use std::ops::{Deref, DerefMut};

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
