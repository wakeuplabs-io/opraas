
pub trait TTestnetNode {
    fn new(chain_id: u32, fork_url: &str, port: u64) -> Self;
    fn start(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
}