FROM --platform=linux/arm64 debian:bookworm-slim

# install necessary dependencies
RUN apt-get update && apt-get install -y \
  curl \
  jq \
  git \
  make \
  bash \
  ca-certificates \
  zip 

RUN curl -L https://github.com/casey/just/releases/download/1.36.0/just-1.36.0-aarch64-unknown-linux-musl.tar.gz | tar xz -C /usr/local/bin

RUN curl -L https://foundry.paradigm.xyz | bash && /root/.foundry/bin/foundryup
ENV PATH="/root/.foundry/bin:$PATH"
RUN foundryup --version nightly-143abd6a768eeb52a5785240b763d72a56987b4a

# Install Go
ENV GO_VERSION=1.22.6
RUN curl -L https://go.dev/dl/go${GO_VERSION}.linux-arm64.tar.gz | tar -C /usr/local -xz

# Add Go to the PATH
ENV PATH="/usr/local/go/bin:$PATH"


WORKDIR /app
COPY . .

# build contracts and go
RUN make submodules
RUN cd /app/packages/contracts-bedrock && just -v build
RUN cd /app && make op-node
RUN chmod +x /app/op-node/bin/op-node

# shared volume paths
ENV IN_DEPLOY_CONFIG=/shared/in/deploy-config.json
ENV OUT_DEPLOY_CONFIG=/shared/out/deploy-config.json
ENV OUT_ADDRESSES=/shared/out/addresses.json
ENV OUT_ALLOCS=/shared/out/allocs-l2.json
ENV OUT_GENESIS=/shared/out/genesis.json
ENV OUT_ROLLUP_CONFIG=/shared/out/rollup-config.json

# DO NOT override. Required by contracts
ENV DEPLOYMENT_OUTFILE=/app/packages/contracts-bedrock/deployments/addresses.json
ENV DEPLOY_CONFIG_PATH=/app/packages/contracts-bedrock/deploy-config/deploy-config.json
ENV STATE_DUMP_PATH=/app/packages/contracts-bedrock/allocs-l2.json 
ENV CONTRACT_ADDRESSES_PATH=${DEPLOYMENT_OUTFILE}

# OVERRIDE
ENV ETH_RPC_URL="http://host.docker.internal:8545"
ENV IMPL_SALT="0x0000000000000000000000000000000000000000000000000000000000000000"
ENV DEPLOYER_ADDRESS="0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
ENV DEPLOYER_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
ENV GAS_MULTIPLIER="130"
ENV SLOW_ARG="--slow"

CMD echo "Updating deploy config..." && \
  cp ${IN_DEPLOY_CONFIG} ${DEPLOY_CONFIG_PATH} && \
  l1GenesisBlockTimestamp=$(printf '0x%x' $(date +%s)) && jq --arg ts "$l1GenesisBlockTimestamp" '.l1GenesisBlockTimestamp = $ts' ${DEPLOY_CONFIG_PATH} > tmp.json && mv tmp.json ${DEPLOY_CONFIG_PATH} && \
  l1StartingBlockTag=$(cast block latest --rpc-url "$ETH_RPC_URL" --json | jq -r ".hash") && jq --arg ts "$l1StartingBlockTag" '.l1StartingBlockTag = $ts' ${DEPLOY_CONFIG_PATH} > tmp.json && mv tmp.json ${DEPLOY_CONFIG_PATH} && \
  echo "{}" > ${DEPLOYMENT_OUTFILE} && \
  echo "Deploying L2 contracts to L1..." && \
  cd /app/packages/contracts-bedrock && forge script scripts/deploy/Deploy.s.sol:Deploy --sig 'runWithStateDump()' --sender "${DEPLOYER_ADDRESS}" --private-key "${DEPLOYER_PRIVATE_KEY}" --gas-estimate-multiplier ${GAS_MULTIPLIER} --rpc-url "${ETH_RPC_URL}" --broadcast ${SLOW_ARG} && \
  cp ${DEPLOYMENT_OUTFILE} ${OUT_ADDRESSES} && \
  cp ${DEPLOY_CONFIG_PATH} ${OUT_DEPLOY_CONFIG} && \
  echo 'Generating L2 genesis allocs' && \
  cd /app/packages/contracts-bedrock && forge script scripts/L2Genesis.s.sol:L2Genesis --sig 'runWithAllUpgrades()' && mv ${STATE_DUMP_PATH} ${OUT_ALLOCS} && \
  echo 'Generating l2 genesis and rollup configs' && \
  /app/op-node/bin/op-node genesis l2 \
      --l1-rpc ${ETH_RPC_URL} \
      --deploy-config ${OUT_DEPLOY_CONFIG} \
      --l2-allocs ${OUT_ALLOCS} \
      --l1-deployments ${OUT_ADDRESSES} \
      --outfile.l2 ${OUT_GENESIS} \
      --outfile.rollup ${OUT_ROLLUP_CONFIG} && \
  zip -j /shared/out/artifacts.zip ${OUT_ADDRESSES} ${OUT_ALLOCS} ${OUT_GENESIS} ${OUT_ROLLUP_CONFIG} ${OUT_DEPLOY_CONFIG}