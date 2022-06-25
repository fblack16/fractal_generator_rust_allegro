pub trait Payload {}

impl Payload for () {}
impl Payload for String {}

pub enum ActionError {
}

pub trait Action {
    fn execute(&mut self) -> Result<(), ActionError>;
}
