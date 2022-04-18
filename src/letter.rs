use std::fmt::Debug;
use std::hash::Hash;

pub trait Letter: Debug + PartialEq + Eq + Hash + Copy {
}

impl Letter for usize {
}

impl Letter for char {
}

impl Letter for fn(i32) -> i32 {
}
