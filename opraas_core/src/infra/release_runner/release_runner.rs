use std::path::Path;

use crate::domain::Release;


pub trait TReleaseRunner {
    fn run(&self, release: &Release, volume: &Path, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>>;
}
