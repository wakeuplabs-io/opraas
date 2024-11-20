pub struct Stack;

pub trait TStackInfraRepository {
    fn pull(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn exists(&self) -> bool;
}