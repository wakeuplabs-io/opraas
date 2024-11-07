
pub trait Initializable {
    fn initialize(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>>;
}
