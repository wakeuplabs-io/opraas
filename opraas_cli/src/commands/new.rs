use async_trait::async_trait;
use log::info;

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

        // create dir
        std::fs::create_dir(&proy_dir)?;

        // create README.md
        let readme_path = proy_dir.join("README.md");
        std::fs::write(&readme_path, README)?;

        // create gitignore
        let gitignore_path = proy_dir.join(".gitignore");
        std::fs::write(&gitignore_path, GITIGNORE)?;

        // create default config
        let config_path = proy_dir.join("config.toml");
        let null_cfg = opraas_core::config::CoreConfig::new_from_null();
        null_cfg.to_toml(&config_path)?;

        // create .env
        let env_path = proy_dir.join(".env");
        std::fs::write(&env_path, ENV_FILE)?;

        println!("✅ Project created at ./{}", self.name);
        println!("🚀 Check the config file and run `opraas setup` to setup the project");

        Ok(())
    }
}

const README: &str = r#"
# Opraas

Optimism Rollup As A Service. Easily deploy and manage rollups with the Optimism stack.
"#;

const GITIGNORE: &str = r#"
.env
"#;

const ENV_FILE: &str = r#"
L1_RPC_URL="https://eth-sepolia.g.alchemy.com/v2/..."
ADMIN_PRIVATE_KEY="5a814bcdce11f289bf252b2a29a85f06e5fe32d05621bcb459a94328859d0c1c"
BATCHER_PRIVATE_KEY="5a814bcdce11f289bf252b2a29a85f06e5fe32d05621bcb459a94328859d0c1c"
PROPOSER_PRIVATE_KEY="5a814bcdce11f289bf252b2a29a85f06e5fe32d05621bcb459a94328859d0c1c"
SEQUENCER_PRIVATE_KEY="5a814bcdce11f289bf252b2a29a85f06e5fe32d05621bcb459a94328859d0c1c"
DEPLOYER_PRIVATE_KEY="5a814bcdce11f289bf252b2a29a85f06e5fe32d05621bcb459a94328859d0c1c"
"#;
