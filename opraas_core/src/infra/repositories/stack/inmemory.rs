use crate::domain::TStackInfraRepository;

pub struct InMemoryStackInfraRepository {
    root: std::path::PathBuf
}

// implementations ================================================

impl InMemoryStackInfraRepository {
    pub fn new(root: &std::path::PathBuf) -> Self {
        Self { root: root.clone() }
    }
}

impl TStackInfraRepository for InMemoryStackInfraRepository {
    fn exists(&self) -> bool {
        self.root.join("infra").exists()
    }

    fn pull(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: download helm infra release from github. Not yet available
        Ok(())
    }
}