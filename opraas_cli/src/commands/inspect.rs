use async_trait::async_trait;

pub struct InspectCommand {
    pub target: String
}

#[async_trait]
impl crate::Runnable for InspectCommand {
    async fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {

        match self.target.as_ref() {
            "contracts" => {
                info!("Inspecting contracts...");
            },
            "infra" => {
                info!("Inspecting infra...");
            },
            _ => return Err("Invalid target".into()),
        }

        Ok(())
    }
}
