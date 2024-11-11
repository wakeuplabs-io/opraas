use crate::domain;

pub struct ArtifactInitializer {
    project: Box<dyn domain::project::TProjectService>,
    source_repository: Box<dyn domain::artifact::TArtifactSourceRepository>,
}


impl domain::artifact::TArtifactInitializerService for ArtifactInitializer {
    fn initialize(
        &self,
        artifact: &domain::artifact::Artifact,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.project.exists(&artifact.source) {
            return Ok(());
        }

        self.source_repository.pull(
            &artifact.release_url,
            &artifact.release_tag,
            &artifact.source,
        )?;

        Ok(())
    }
}
