use crate::word::Word;

pub trait Letter {
    fn combine(&self, other: &Self) -> Word<Self> where Self: Sized;
}

impl Letter for usize {
    fn combine(&self, other: &Self) -> Word<Self>
    {
        let mut word = Word::new();
        word.push(self + other);
        word
    }
}
