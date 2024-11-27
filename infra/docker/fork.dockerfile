FROM node:18-alpine

WORKDIR /app

RUN echo -e '{ "name": "fork-node", "devDependencies": { "hardhat": "2.22.13" }}' > package.json
RUN echo -e 'module.exports = {\n  solidity: "0.8.0",\n  networks: {\n    hardhat: {\n      chainId: Number(process.env.CHAIN_ID),\n    }\n  }\n};' > hardhat.config.js

RUN npm install

EXPOSE 8545

# Override
ENV CHAIN_ID=1
ENV FORK_URL=https://eth-mainnet.alchemyapi.io/v2/...

CMD npx hardhat node --fork $FORK_URL
