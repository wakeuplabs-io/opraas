use crate::domain::artifact::Artifact;

pub trait TArtifactBuilder {
    fn build_artifact(&self, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
}
