
pub trait TTestnetNode {
    fn start(chain_id: u64, fork_url: &str, port: u64) -> Result<(), Box<dyn std::error::Error>>;
    fn stop() -> Result<(), Box<dyn std::error::Error>>;
}