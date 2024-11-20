pub trait TVersionControl {
    fn init(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn stage(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn commit(&self, filepath: &str, message: &str) -> Result<(), Box<dyn std::error::Error>>;
}
