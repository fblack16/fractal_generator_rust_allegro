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
