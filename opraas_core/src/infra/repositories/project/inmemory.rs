use crate::domain;

pub struct InMemoryProjectRepository;

impl InMemoryProjectRepository {
    pub fn new() -> Self {
        Self
    }
}

impl domain::project::TProjectRepository for InMemoryProjectRepository {
    fn exists(&self, filepath: &std::path::PathBuf) -> bool {
        filepath.exists()
    }

    fn write(&self, filepath: &std::path::PathBuf, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = filepath.parent() {
            std::fs::create_dir_all(parent)?;  // Creates all missing directories in the path
        }

        std::fs::write(filepath, content)?;

        Ok(())
    }
    
}