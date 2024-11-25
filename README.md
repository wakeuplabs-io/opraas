
# OP-RUAAS - Optimism Rollup as a service

Optimism Rollup As A Service. Easily deploy and manage rollups with the Optimism stack.

## Makefile commands

- `make run ....` -> compile and run cli on the fly
- `make format`
- `make lint`
- `make release-{windows/apple/linux}` -> Creates binaries and zip releases within releases folder.

## Opraas cli

### Commands

Usage: opruaas_cli [OPTIONS] <COMMAND>

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
opruaas_cli --name my-chain new && cd my-chain

# 2. Fill up config.toml and .env

# 3. Pull sources with init (target can be all|batcher|node|geth|contracts)
opruaas_cli --target contracts init

# 4. Build images with 
opruaas_cli --target contracts build 

# 5. Finally when ready release. It's important you have docker already configured with enough permissions to push to the repo you want to release to
opruaas_cli --target contracts release 
```

### Test releases with dev

```bash
# 1. Just run dev command... We'll prompt you about which release to use
# We'll fork the l1 you have in .env so make sure to have a valid rpc. As per wallets we'll replace your values with mock wallets already funded.
opruaas_cli dev
```

### Deploy contracts/infra/all

```bash
# 1. Just run dev command... We'll prompt you about which release to use
opruaas_cli --target all --name my-prod-deployment deploy 
```

### Npm distribution 

1. Within makefile update `RELEASE_VERSION`
2. Run `make release-{windows/apple/linux}`
3. Upload assets to github release named `v{RELEASE_VERSION}`
4. Bump npm package version to match the release
5. `npm run publish --access public`



