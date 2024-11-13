use crate::domain;



pub struct InMemoryDeploymentRepository;

// implementations ====================================

impl InMemoryDeploymentRepository {
    pub fn new() -> Self {
        Self
    }
}

impl domain::deployment::TDeploymentRepository for InMemoryDeploymentRepository {
    fn find(&self, root: &std::path::PathBuf, name: String) -> Option<domain::Deployment> {
        // TODO:
        
        None
    }

    fn save(&self, root: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // TODO:

        Ok(())
    }
}