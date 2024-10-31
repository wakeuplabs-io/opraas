use async_trait::async_trait;

pub struct NewCommand {
    pub name: String,
}

#[async_trait]
impl crate::Runnable for NewCommand {
    async fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let cwd = std::env::current_dir()?;
        let proy_dir = cwd.join(&self.name);
        info!("Creating new project at {}...", proy_dir.display());

        if proy_dir.exists() {
            return Err("Directory already exists".into());
        }

        std::fs::create_dir(&proy_dir)?;
        
        let config_path = proy_dir.join("config.toml");
        let null_cfg = opraas_core::config::CoreConfig::new_from_null();
        null_cfg.to_toml(&config_path)?;
        
        println!("✅ Project created at ./{}", self.name);
        println!("🚀 Check the config file and run `opraas setup` to setup the project");

        Ok(())
    }
}
