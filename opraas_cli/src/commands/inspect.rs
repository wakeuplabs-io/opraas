use async_trait::async_trait;
use crate::console;

pub struct InspectCommand {
    pub target: String
}

#[async_trait]
impl crate::Runnable for InspectCommand {
    async fn run(&self, cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {

        match self.target.as_ref() {
            "contracts" => {
                console::info("Inspecting contracts...");
            },
            "infra" => {
                console::info("Inspecting infra...");
            },
            _ => return Err("Invalid target".into()),
        }

        Ok(())
    }
}

