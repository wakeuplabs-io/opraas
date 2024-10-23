# Research & Discovery

# Porpoise

The goal of this project is to enable users to easily deploy and manage their own rollups through cloud providers. The service will include a Command Line Interface (CLI) and a web-based development console for management. By eliminating vendor lock-in, we empower users to fully own their rollups and switch cloud providers with minimal effort.

# Summary

# Problem statement + Context

# Alternatives considered

### 0xFableOrg/roll-op

Pros:

- Functional implementation; it’s the only one we’ve been able to run locally so far.
- Implements BlockScout as a block explorer out of the box.

Cons:

- Written in Python, which could lead us to use Python for our entire development—a choice we want to avoid, as it often results in heavy lower-quality code.
- Code quality is low; the structure is somewhat messy and poorly organized.
- Tied to a single version of the source code, with no easy way to adapt to additional versions.
- Deploys contracts from the OP node, contrary to the advice in the documentation.
- Lacks bridge deployment capabilities.
- Contains excessive dependencies.
- Does not address infrastructure deployment and management.

Source:

- https://github.com/0xFableOrg/roll-op

### upnodedev/opstack-compose

Pros:

- DevOps solution using Docker Compose and AWS.
- Upnode focuses on blockchain infrastructure, providing more value.
- Implements BlockScout as a block explorer out of the box.
- Offers deployment examples that we can draw inspiration from.

Cons:

- Does not work out of the box.
- Hardcoded build process limits flexibility regarding version attachment.
- Contracts are deployed from the same release used to build the binaries, which can create complications.

Source:

- https://github.com/upnodedev/opstack-compose

### optimism/op-deployer

Pros:

- Developed and maintained by Optimism itself
- Utilizes the [OPCM](https://github.com/ethereum-optimism/design-docs/blob/main/protocol/op-contracts-manager-arch.md) which simplifies contract deployments significantly.

Cons:

- Still in the early stages for our use; limited documentation and usage examples.
- Primarily focused on contract deployment.
- Currently relies on a deployment script.
- Does not address infrastructure deployment and management.

Source:

- https://docs.optimism.io/builders/chain-operators/tools/op-deployer

# Proposed solutions

We have decided to develop our own CLI and infrastructure tooling to create a more flexible deployment process and address some of the inconveniences we encountered while testing existing solutions. Given the challenges in getting current solutions to work easily and the relatively short time required to replicate them, we aim for our development to also meet new requirements, including a user interface and scalable deployment.

## Decisions and drivers

General

- We chose Rust for its performance, cost efficiency, and growing popularity, ensuring reliable and scalable solutions.

CLI

- CLI for local self-service
- The CLI will be an implementation of a core Rust package that manages the business logic for creating the blockchain. This approach allows us to maintain testable code while also enabling multiple client implementations, including the server itself.
- We'll distribute the binaries through github releases and make them widely available with an npm package.
- To allow for a more flexible build process we'll allow the user to specify the steps in the config file in the shape of:

  ```toml
  [sources.op_node]
  release_tag = "op-node/v1.3.1"
  build = ["make op-node"]

  [sources.op_contracts]
  release_tag = "op-contracts/v1.6.0"
  build = ["pnpm install", "pnpm build", "cd packages/bedrock-contracts", "forge install", "..."]
  ```

  Note this also gives flexibility for patches or cherry picks on top of releases like roll-op does.

- Artifacts will be downloaded form github releases.
- To reduce dependencies the user needs to manage we can provide a docker image with the binaries already available so with some volume mapping we reduce the boilerplate setup to just docker.

Infrastructure

- We will deploy using `Helm` based on `opstack-compose`. Leveraging `Kubernetes` enables us to expand the number of providers that can support us while providing the scalability we need. This approach will simplify the user experience during deployment. We may incorporate `Terraform` to further facilitate the process.
- Cloud service App: UI + Lambda API implementing Core

Block explorer

- Block explorer. For the block explorer [Blockscout](https://www.blockscout.com/) appears to be the best choice. BlockScout is an open-source block explorer that supports OP Stack chains, is recommended by the Optimism documentation, and is widely implemented in many current solutions.

# Risk and uncertainties

1. It must be sufficiently extensible to handle new features.
2. Upgrades management
3. Contemplate testing environments.
4. Avoid overloading user with enviroment setup complexities.
5. Testnet tokens availability and proper config setup
