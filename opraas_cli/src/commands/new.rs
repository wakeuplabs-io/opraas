use async_trait::async_trait;

use crate::console::{print_info, print_success};

pub struct NewCommand {
    pub name: String,
}

impl NewCommand {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[async_trait]
impl crate::Runnable for NewCommand {
    async fn run(&self, _cfg: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
        let cwd = std::env::current_dir()?;
        let proy_dir = cwd.join(&self.name);

        if proy_dir.exists() {
            return Err("Directory already exists".into());
        }

        // create dir
        std::fs::create_dir(&proy_dir)?;
        std::fs::write(&proy_dir.join("README.md"), README)?;
        std::fs::write(&proy_dir.join(".gitignore"), GITIGNORE)?;
        std::fs::write(&proy_dir.join(".env"), ENV_FILE)?;

        // create default config
        let null_cfg = opraas_core::config::CoreConfig::new_from_null();
        null_cfg.to_toml(&proy_dir.join("config.toml"))?;

        print_success(&format!("✅ Project created at ./{}", self.name));
        print_info("🚀 Check the config file and run `opraas setup` to setup the project");

        Ok(())
    }
}

const README: &str = r#"
# Opraas

Optimism Rollup As A Service. Easily deploy and manage rollups with the Optimism stack.

## Commands

- `opraas new <name>` to create a new project
- `opraas setup` to setup a new project
- `opraas build <target>` to compile sources and create docker images for it
- `opraas deploy <target> <name>` to deploy your blockchain. Target must be one of: contracts, infra, all
- `opraas dev` to spin up local dev environment
- `opraas version` to check the opraas version

## Instructions

1. Create a new project with `opraas new <name>`
2. Update `<name>/config.toml` and `<name>/.env` to match your needs
2. Run `opraas setup` to download the code for your chain
3. Run `opraas build <target>` to compile sources and create docker images for them
4. Run `opraas deploy <target> <name>` to deploy your blockchain. Target must be one of: contracts, infra, all
5. Run `opraas dev` to spin up local dev environment
6. Run `opraas version` to check the opraas version


## Notes

...
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
