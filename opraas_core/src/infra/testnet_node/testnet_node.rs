
pub trait TTestnetNode {
    fn start(&self, chain_id: u32, fork_url: &str, port: u64) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&self);
}