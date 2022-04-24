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
