
pub trait TTestnetNode: Send + Sync {
    fn start(&self, chain_id: u64, fork_url: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
}
