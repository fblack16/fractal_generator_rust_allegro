use std::{collections::HashMap, hash::Hash};

pub trait Word {
    type Owned;
    fn first_subword(&self, valid_subwords: &[&Self]) -> Option<&Self>;
    fn subwords(&self, valid_subwords: &[&Self]) -> Vec<&Self>;
    fn contains(&self, word: &Self) -> bool;
    fn apply_relacements(&self, replacements: HashMap<Self::Owned, Self::Owned>) -> Self::Owned;
    fn apply_semantics(&self, semantics: HashMap<Self::Owned, fn(&Self)>);
}

impl<T> Word for [T]
where
    T: Clone + PartialEq + Eq + Hash,
{
    type Owned = Vec<T>;
    fn first_subword(&self, valid_subwords: &[&Self]) -> Option<&Self> {
        let mut longest_subword: &[T] = &[];

        for &subword in valid_subwords {
            if !subword.is_empty() && self.starts_with(subword) {
                if longest_subword.starts_with(subword) {
                    continue;
                } else {
                    longest_subword = &self[..subword.len()];
                }
            }
        }

        if longest_subword.is_empty() {
            return None;
        }

        return Some(longest_subword);
    }

    fn subwords(&self, valid_subwords: &[&Self]) -> Vec<&Self> {
        let mut subwords = Vec::new();
        let mut start = 0;

        while let Some(word) = self[start..].first_subword(valid_subwords) {
            subwords.push(word);
            start += word.len();
        }

        return subwords;
    }

    fn contains(&self, word: &Self) -> bool {
        if self.is_empty() || word.is_empty() {
            return false;
        }

        let mut is_contained = false;
        for (index, elem) in self.iter().enumerate() {
            if elem == &word[0] && &self[index..index+word.len()] == word {
                is_contained = true;
            }
        }

        return is_contained;
    }

    fn apply_relacements(&self, replacements: HashMap<Self::Owned, Self::Owned>) -> Self::Owned {
        let valid_subwords: Vec<&Self> = replacements.keys().map(|word| &word[..]).collect();
        let word = self.subwords(&valid_subwords[..]).into_iter()
            .map(|word| {
                if let Some(replacement) = replacements.get(word) {
                    replacement
                } else {
                    word
                }
            })
            .map(|word| word.to_owned())
            .flatten()
            .collect();
        word
    }

    fn apply_semantics(&self, semantics: HashMap<Self::Owned, fn(&Self)>) {
        let valid_subwords: Vec<&Self> = semantics.keys().map(|word| &word[..]).collect();
        for word in self.subwords(&valid_subwords[..]){
            if let Some(action) = semantics.get(word) {
                action(word);
            }
        }
    }
}

#[cfg(test)]
mod test {

    mod first_subword {
        use crate::word_slice::Word;

        #[test]
        fn return_none_for_empty_word() {
            let word = vec![];
            let valid_subwords = vec![
                &[1][..],
            ];
            let result = word.first_subword(&valid_subwords[..]);
            assert!(result.is_none());
        }

        #[test]
        fn return_none_if_valid_word_is_empty() {
            let word = vec![1, 2, 3];
            let valid_subwords: Vec<&[i32]> = vec![
                &[],
            ];
            let result = word.first_subword(&valid_subwords[..]);
            assert!(result.is_none());
        }

        #[test]
        fn context_free() {
            let word = vec![1, 2, 3, 4, 5];
            let valid_subwords = vec![
                &[1][..],
            ];
            let result = word.first_subword(&valid_subwords[..]);
            assert!(result.is_some());
            assert_eq!(result.unwrap(), &[1][..]);
        }

        #[test]
        fn context_dependent() {
            let word = vec![1, 2, 3, 4, 5];
            let valid_subwords = vec![
                &[1][..],
                &[1, 2][..],
            ];
            let result = word.first_subword(&valid_subwords[..]);
            assert!(result.is_some());
            assert_eq!(result.unwrap(), &[1, 2][..]);
        }
    }

    mod subwords {
        use crate::word_slice::Word;

        #[test]
        fn return_no_subwords_for_empty_word() {
            let word: Vec<i32> = vec![];
            let valid_subwords = vec![
                &[1][..]
            ];
            let subwords = word.subwords(&valid_subwords);
            assert_eq!(subwords, vec![] as Vec<&[i32]>);
        }

        #[test]
        fn return_no_subwords_for_empty_list_of_valid_words() {
            let word = vec![1, 2, 3, 4, 5];
            let valid_subwords = vec![];
            let subwords = word.subwords(&valid_subwords);
            assert_eq!(subwords, vec![] as Vec<&[i32]>);
        }

        #[test]
        fn context_free() {
            let word = vec![1, 2, 3, 4, 5];
            let first = [1, 2];
            let second = [3];
            let third = [4, 5];
            let valid_subwords = vec![
                &first[..],
                &second[..],
                &third[..],
            ];
            let subwords = word.subwords(&valid_subwords);
            assert_eq!(subwords, vec![&first[..], &second[..], &third[..]]);
        }

        #[test]
        fn context_dependent() {
            let word = vec![1, 2, 3, 4, 5];
            let first = [1, 2];
            let second = [1, 2, 3];
            let third = [4, 5];
            let valid_subwords = vec![
                &first[..],
                &second[..],
                &third[..],
            ];
            let subwords = word.subwords(&valid_subwords);
            assert_eq!(subwords, vec![&second[..], &third[..]]);
        }
    }
}
