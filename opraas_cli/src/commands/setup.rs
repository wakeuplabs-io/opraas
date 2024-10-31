use crate::console::ConsoleProgressTracker;
use async_trait::async_trait;
use opraas_core::artifacts::build::BuildArtifact;

pub struct SetupCommand;

#[async_trait]
impl crate::Runnable for SetupCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let core_cfg = cfg.build_core()?;

        if !cfg.tree.src.batcher.exists() {
            opraas_core::artifacts::build::batcher::BatcherBuildArtifact
                .download(
                    &core_cfg,
                    &ConsoleProgressTracker::new_progress_bar("⏳ Downloading batcher...", "✅ Downloaded!"),
                )
                .await?;
        }

        Ok(())
    }
}
