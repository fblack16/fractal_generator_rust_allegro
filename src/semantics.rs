use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use crate::word::Word;
use crate::letter::Letter;

pub trait Payload {}

pub struct Semantics<L, P>
where
    L: Letter,
    P: Payload,
{
    hash_map: HashMap<Word<L>, fn(&Word<L>, &P)>,
}

impl<L, P> Semantics<L, P>
where
    L: Letter,
    P: Payload,
{
    pub fn new() -> Self {
        Semantics {
            hash_map: HashMap::new(),
        }
    }
}

impl<L, P> Deref for Semantics<L, P>
where
    L: Letter,
    P: Payload,
{
    type Target = HashMap<Word<L>, fn(&Word<L>, &P)>;
    fn deref(&self) -> &Self::Target {
        &self.hash_map
    }
}

impl<L, P> DerefMut for Semantics<L, P>
where
    L: Letter,
    P: Payload,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hash_map
    }
}
