use crate::{
    config::CoreConfig,
    domain::{self, Project, Stack, TStackInfraRepository},
    infra::{self, repositories::stack_infra::GitStackInfraRepository},
};

pub struct CreateProjectService {
    repository: Box<dyn domain::project::TProjectRepository>,
    version_control: Box<dyn infra::version_control::TVersionControl>,
    stack_infra_repository: Box<dyn TStackInfraRepository>,
}

pub trait TCreateProjectService {
    fn create(&self, root: &std::path::PathBuf) -> Result<Project, Box<dyn std::error::Error>>;
}

impl CreateProjectService {
    pub fn new() -> Self {
        Self {
            repository: Box::new(infra::repositories::project::InMemoryProjectRepository::new()),
            version_control: Box::new(infra::version_control::GitVersionControl::new()),
            stack_infra_repository: Box::new(GitStackInfraRepository::new()),
        }
    }
}

impl TCreateProjectService for CreateProjectService {
    fn create(&self, root: &std::path::PathBuf) -> Result<Project, Box<dyn std::error::Error>> {
        if root.exists() {
            return Err("Directory already exists".into());
        }

        let project = Project::new_from_root(root.to_path_buf());

        self.repository
            .write(&project, &root.join("README.md"), README)?;
        self.repository
            .write(&project, &root.join(".gitignore"), GITIGNORE)?;
        self.repository
            .write(&project, &root.join(".env"), ENV_FILE)?;
        self.repository
            .write(&project, &root.join(".env.sample"), ENV_FILE)?;
        self.repository.write(
            &project,
            &root.join("config.toml"),
            &toml::to_string(&CoreConfig::default()).unwrap(),
        )?;

        // pull stack infra
        self.stack_infra_repository.pull(&Stack::new(
            project.infra.helm.clone(),
            project.infra.aws.clone(),
            None,
        ))?;

        // initialize git and create first commit
        self.version_control.init(&root.to_str().unwrap())?;
        self.version_control.stage(&root.to_str().unwrap())?;
        self.version_control
            .commit(&root.to_str().unwrap(), "First commit")?;

        Ok(project)
    }
}

const README: &str = r#"
# Opruaas

Optimism Rollup As A Service. Easily deploy and manage rollups with the Optimism stack.

## Commands

- `opraas new <name>` -> Creates a new project at `<name>`
- `opraas init` -> Pulls sources as per your config.toml
- `opraas build` -> Compiles sources and creates docker images for them
- `opraas release` -> Tags and pushes already built docker images to the registry
- `opraas dev` -> Spins up local dev environment
- `opraas deploy` -> Deploys your blockchain

## Instructions

1. Create a new project with `opraas new <name>`
2. Update `<name>/config.toml` and `<name>/.env` to match your needs
3. If you want to build your own artifacts
    1. Run `opraas init <target>` to pull sources as per your config.toml
    2. Run `opraas build <target>` to compile sources and create docker images for them
    3. Run `opraas release <target>` to tag and push docker images
4. If you want to use pre-built artifacts
    1. Run `opraas dev` to spin up local dev environment
    2. Run `opraas deploy <target> <name>` to deploy your blockchain. Target must be one of: contracts, infra, all


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
CHALLENGER_PRIVATE_KEY="5a814bcdce11f289bf252b2a29a85f06e5fe32d05621bcb459a94328859d0c1c"
"#;
