pub trait Semantics
{
    fn execute(&self);
}

pub enum Koch {
    Forward,
    TurnLeft,
    TurnRight,
}

impl Semantics for Koch {
    fn execute(&self) {
        match self {
            Self::Forward => { println!("Forward "); },
            Self::TurnLeft => { println!("TurnLeft "); },
            Self::TurnRight => { println!("TurnRight "); },
        }
    }
}

