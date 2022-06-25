pub trait Payload {}

impl Payload for () {}

pub enum ActionError {
}

pub trait Action {
    fn execute(&mut self) -> Result<(), ActionError>;
}
