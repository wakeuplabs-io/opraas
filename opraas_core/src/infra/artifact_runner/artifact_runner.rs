use crate::domain::artifact::Artifact;

pub trait TArtifactRunner {
    fn run_artifact(&self, artifact: &Artifact, volume: &str, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>>;
}
