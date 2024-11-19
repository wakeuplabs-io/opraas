use crate::{config::CoreConfig, domain, infra};

pub struct CreateProjectService {
    repository: Box<dyn domain::project::TProjectRepository>,
    version_control: Box<dyn infra::version_control::TVersionControl>
}

pub trait TCreateProjectService {
    fn create(&self, root: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>>;
}

impl CreateProjectService {
    pub fn new() -> Self {
        Self {
            repository: Box::new(infra::repositories::project::InMemoryProjectRepository::new()),
            version_control: Box::new(infra::version_control::GitVersionControl::new()),
        }
    }
}

impl TCreateProjectService for CreateProjectService {
    fn create(
        &self,
        root: &std::path::PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if root.exists() {
            return Err("Directory already exists".into());
        }

        self.repository.write(&root.join("README.md"), README)?;
        self.repository.write(&root.join(".gitignore"), GITIGNORE)?;
        self.repository.write(&root.join(".env"), ENV_FILE)?;
        self.repository.write(&root.join(".env.sample"), ENV_FILE)?;
        self.repository.write(
            &root.join("config.toml"),
            &toml::to_string(&CoreConfig::default()).unwrap(),
        )?;

        // initialize git and create first commit
        self.version_control.init(&root.to_str().unwrap())?;
        self.version_control.stage(&root.to_str().unwrap())?;
        self.version_control
            .commit(&root.to_str().unwrap(), "Initial commit")?;

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
L1_RPC_URL="https://eth-mainnet.g.alchemy.com/v2/..."
ADMIN_PRIVATE_KEY="5a814bcdce11f289bf252b2a29a85f06e5fe32d05621bcb459a94328859d0c1c"
BATCHER_PRIVATE_KEY="5a814bcdce11f289bf252b2a29a85f06e5fe32d05621bcb459a94328859d0c1c"
PROPOSER_PRIVATE_KEY="5a814bcdce11f289bf252b2a29a85f06e5fe32d05621bcb459a94328859d0c1c"
SEQUENCER_PRIVATE_KEY="5a814bcdce11f289bf252b2a29a85f06e5fe32d05621bcb459a94328859d0c1c"
DEPLOYER_PRIVATE_KEY="5a814bcdce11f289bf252b2a29a85f06e5fe32d05621bcb459a94328859d0c1c"
"#;
