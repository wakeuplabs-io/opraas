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
        std::fs::create_dir_all(root)?;

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
# Opruaas - Optimism Rollup as a service

Optimism Rollup As A Service. Easily deploy and manage rollups with the Optimism stack.

## Makefile commands

- `make format`
- `make lint`
- `make release-{windows/apple/linux}` -> Creates binaries and zip releases within releases folder.

## Opruaas cli

Install with `npm i -g @wakeuplabs/opruaas`

### Commands

Usage: opruuas [OPTIONS] <COMMAND>

Commands:
-  `new`      Create new project, template config file and folders
-  `init`     Initialize a new project
-  `build`    Compile sources and create docker images for it
-  `release`  Tags and pushes already built docker images to the registry for usage in the deployment
-  `dev`      Spin up local dev environment
-  `deploy`   Deploy your blockchain. Target must be one of: contracts, infra, all
-  `inspect`  Get details about the current deployment. Target must be one of: contracts, infra
-  `help`     Print this message or the help of the given subcommand(s)

Options:
-  `-q`, `--quiet`    Suppress logging output
-  `-h`, `--help`     Print help
-  `-V`, `--version`  Print version

### Create new project and build releases from source

```bash
# 1. create your project
npx opruaas new my-chain && cd my-chain

# 2. Fill up config.toml and .env

# 3. Pull sources with init (target can be all|batcher|node|geth|contracts)
npx opruaas --quiet init contracts

# 4. Build images with 
npx opruaas build contracts

# 5. Finally when ready release. It's important you have docker already configured with enough permissions to push to the repo you want to release to
npx opruaas release contracts
```

### Test releases with dev

```bash
# 1. Just run dev command... We'll prompt you about which release to use
# Don't worry about wallets, we'll override with testing ones.
npx opruaas -v dev
```

Once all deployments are up and running it may take some time for it to be reactive, for rpc to respond and for explorer to finish indexing and start showing your transactions.

If you have cast installed some of these commands may help you to give it a try

```bash
cast chain-id --rpc-url http://localhost:80/rpc

cast balance 0x3fAB184622Dc19b6109349B94811493BF2a45362 --rpc-url http://localhost:80/rpc

cast send \
  --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 \
  --private-key ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
  --rpc-url http://localhost:80/rpc \
  --value 1ether \
  0x3fAB184622Dc19b6109349B94811493BF2a45362
```

On l1 and l2 all these wallets will be funded by default (we automatically set `fund_dev_accounts` to `true` for dev mode)

```
0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 with 10000 ETH
0x70997970C51812dc3A010C7d01b50e0d17dc79C8 with 10000 ETH
0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC with 10000 ETH
0x90F79bf6EB2c4f870365E785982E1f101E93b906 with 10000 ETH
0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65 with 10000 ETH
0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc with 10000 ETH
0x976EA74026E726554dB657fA54763abd0C3a0aa9 with 10000 ETH
0x14dC79964da2C08b23698B3D3cc7Ca32193d9955 with 10000 ETH
0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f with 10000 ETH
0xa0Ee7A142d267C1f36714E4a8F75612F20a79720 with 10000 ETH
0xBcd4042DE499D14e55001CcbB24a551F3b954096 with 10000 ETH
0x71bE63f3384f5fb98995898A86B02Fb2426c5788 with 10000 ETH
0xFABB0ac9d68B0B445fB7357272Ff202C5651694a with 10000 ETH
0x1CBd3b2770909D4e10f157cABC84C7264073C9Ec with 10000 ETH
0xdF3e18d64BC6A983f673Ab319CCaE4f1a57C7097 with 10000 ETH
0xcd3B766CCDd6AE721141F452C550Ca635964ce71 with 10000 ETH
0x2546BcD3c84621e976D8185a91A922aE77ECEc30 with 10000 ETH
0xbDA5747bFD65F08deb54cb465eB87D40e51B197E with 10000 ETH
0xdD2FD4581271e230360230F9337D5c0430Bf44C0 with 10000 ETH
0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199 with 10000 ETH
0x09DB0a93B389bEF724429898f539AEB7ac2Dd55f with 10000 ETH
0x02484cb50AAC86Eae85610D6f4Bf026f30f6627D with 10000 ETH
0x08135Da0A343E492FA2d4282F2AE34c6c5CC1BbE with 10000 ETH
0x5E661B79FE2D3F6cE70F5AAC07d8Cd9abb2743F1 with 10000 ETH
0x61097BA76cD906d2ba4FD106E757f7Eb455fc295 with 10000 ETH
0xDf37F81dAAD2b0327A0A50003740e1C935C70913 with 10000 ETH
0x553BC17A05702530097c3677091C5BB47a3a7931 with 10000 ETH
0x87BdCE72c06C21cd96219BD8521bDF1F42C78b5e with 10000 ETH
0x40Fc963A729c542424cD800349a7E4Ecc4896624 with 10000 ETH
0x9DCCe783B6464611f38631e6C851bf441907c710 with 10000 ETH
```

You'll find:
- L1 rpc available at http://localhost:8545
- L2 rpc available at http://localhost:80/rpc
- Explorer available at http://localhost:80

### Deploy contracts/infra/all

Make sure to have a properly configured your toml config.

```bash
# Recommended -v for verbose deployment as it may take a while to get it all up and running
npx opruaas -v deploy all --name my-prod-deployment
```

Use `--deploy-deployer` in case l1 chain doesn't have one already, for most popular l1 chains you shouldn't worry about it

This will generate `deployments/my-prod-deployment` folder where you can find the generated artifacts. You can manually inspect them or use the `inspect` command for this. It's important you don't loose these files as they're needed to run your chain.

"#;

const GITIGNORE: &str = r#"
.env
"#;

const ENV_FILE: &str = r#"
L1_RPC_URL="https://eth-mainnet.g.alchemy.com/v2/..."
ADMIN_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
BATCHER_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
PROPOSER_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
SEQUENCER_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
DEPLOYER_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
CHALLENGER_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
"#;
