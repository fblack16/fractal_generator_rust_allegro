pub trait Letter: Sized {
    fn replacement(&self) -> Option<Vec<Self>>;
}

pub enum MyLetters {
    Forward,
    TurnLeft,
    TurnRight,
}

impl Letter for MyLetters {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            Self::Forward => {
                None
            },
            Self::TurnLeft => {
                None
            },
            Self::TurnRight => {
                None
            }
        }
    }
}
