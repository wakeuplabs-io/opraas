use crate::domain::Release;


pub trait TReleaseRunner {
    fn run(&self, release: &Release, volume: &str, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>>;
}
