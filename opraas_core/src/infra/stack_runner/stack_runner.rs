use crate::domain::Stack;

pub trait TStackRunner {
    fn run(&self, stack: &Stack) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
}
