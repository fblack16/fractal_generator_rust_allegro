use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

pub struct ReplacementRules<K, V> {
    hash_map: HashMap<K, V>,
}

impl<K, V> ReplacementRules<K, V>
{
    pub fn new() -> Self {
        ReplacementRules {
            hash_map: HashMap::new(),
        }
    }
}

impl<K, V> Deref for ReplacementRules<K, V>
{
    type Target = HashMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.hash_map
    }
}

impl<K, V> DerefMut for ReplacementRules<K, V>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hash_map
    }
}
