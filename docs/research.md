# Research & Discovery

## Existing similar tools and benefits to our project

### 0xFableOrg/roll-op
- Rollup customization with config file


### upnodedev/opstack-compose
- Dev ops solution (docker-compose + aws)
- Upnode focuses on blockchain infrastructure (more value)
- Has a block explorer

## Decisions and drivers

### Developed on Rust
- Fast, lightweight and compiled
- Reduces computing costs
- Increasing popularity
 
## Strategy
- Core isolated from CLI and App packages enables for Core functionalities testing.

- CLI for local self-service

- Deploy with helm on top of `opstack-compose`'s docker-compose
	- Networking already solved
	- Cloud provider-agnostic
	- Provides scalability
    - Eases deploy process

- Cloud service App: UI + Lambda API implementing Core
	- Rust lambdas are cheap and scale easily