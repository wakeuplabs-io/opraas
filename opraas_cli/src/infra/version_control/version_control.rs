

pub trait TVersionControl: Send + Sync {
    fn tag_release(&self, git_path: &str, release_tag: &str) -> Result<(), Box<dyn std::error::Error>>;
}
