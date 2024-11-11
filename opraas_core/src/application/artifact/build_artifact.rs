pub trait TArtifactBuilderService {
    fn build(&self, cfg: &Config, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
    fn release(&self, cfg: &Config, artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>>;
}