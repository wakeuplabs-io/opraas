use std::path::PathBuf;
use mockall::automock;

pub struct Fs;

#[automock]
pub trait Filesystem: Send + Sync {
    fn exists(&self, path: &PathBuf) -> bool;
}

impl Fs {
    pub fn new() -> Self {
        Self
    }
}

impl Filesystem for Fs {
    fn exists(&self, path: &PathBuf) -> bool {
        path.exists()
    }
}
