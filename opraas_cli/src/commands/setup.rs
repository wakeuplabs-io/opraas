use crate::console::ConsoleProgressTracker;
use async_trait::async_trait;
use opraas_core::artifacts::build::BuildArtifact;

pub struct SetupCommand;

#[async_trait]
impl crate::Runnable for SetupCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let core_cfg = cfg.build_core()?;

        let batcher_progress =
            ConsoleProgressTracker::new("⏳ Downloading batcher...");
        if !cfg.tree.src.batcher.exists() {
            opraas_core::artifacts::build::batcher::BatcherBuildArtifact
                .download(&core_cfg, &batcher_progress)
                .await?;
        }
        batcher_progress.finish("✅ Batcher ready");

        Ok(())
    }
}
