
# Opraas

Optimism Rollup As A Service. Easily deploy and manage rollups with the Optimism stack.

## Makefile commands

- `make run ....` -> compile and run cli on the fly
- `make format`
- `make lint`
- `make release-{windows/apple/linux}` -> Creates binaries and zip releases within releases folder.

## Opraas cli

### Commands

- `setup` -> Downloads sources and creates binaries for them.
- `deploy contracts` -> Creates deploy config from user config and deploys contracts.
- WIP, more commands coming.

### Npm distribution 

1. Within makefile update `RELEASE_VERSION`
2. Run `make release-{windows/apple/linux}`
3. Upload assets to github release named `v{RELEASE_VERSION}`
4. Bump npm package version to match the release
5. `npm run publish --access public`



