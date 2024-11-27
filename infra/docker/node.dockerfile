ARG TARGET_BASE_IMAGE=alpine:3.20
ARG BUILDPLATFORM=arm64

FROM --platform=$BUILDPLATFORM golang:1.22.7-alpine3.20 AS builder

RUN apk add --no-cache make gcc musl-dev linux-headers git jq bash

# We copy the go.mod/sum first, so the `go mod download` does not have to re-run if dependencies do not change.
COPY ./go.mod /app/go.mod
COPY ./go.sum /app/go.sum

WORKDIR /app

# warm-up the cache
RUN --mount=type=cache,target=/go/pkg/mod --mount=type=cache,target=/root/.cache/go-build go mod download

COPY . /app

# GetStorageAt by block number instead of hash to increase rpc support.
RUN <<EOF
echo "diff --git a/op-service/sources/eth_client.go b/op-service/sources/eth_client.go" >> fix.patch
echo "index 149a0c0..0fbfe85 100644" >> fix.patch
echo "--- a/op-service/sources/eth_client.go" >> fix.patch
echo "+++ b/op-service/sources/eth_client.go" >> fix.patch
echo "@@ -351,7 +351,12 @@ func (s *EthClient) GetStorageAt(ctx context.Context, address common.Address, st" >> fix.patch
echo " // The storage slot value is verified against the state-root of the given block if we do not trust the RPC provider, or directly retrieved without proof if we do trust the RPC." >> fix.patch
echo " func (s *EthClient) ReadStorageAt(ctx context.Context, address common.Address, storageSlot common.Hash, blockHash common.Hash) (common.Hash, error) {" >> fix.patch
echo " 	if s.trustRPC {" >> fix.patch
echo "-		return s.GetStorageAt(ctx, address, storageSlot, blockHash.String())" >> fix.patch
echo "+		// get blocknumber for hash" >> fix.patch
echo "+		block, err := s.InfoByHash(ctx, blockHash)" >> fix.patch
echo "+		if err != nil {" >> fix.patch
echo "+			return common.Hash{}, fmt.Errorf(\"failed to retrieve state root of block %s: %w\", blockHash, err)" >> fix.patch
echo "+		}" >> fix.patch
echo "+		return s.GetStorageAt(ctx, address, storageSlot, fmt.Sprintf(\"0x%x\", block.NumberU64()))" >> fix.patch
echo " 	}" >> fix.patch
echo " 	block, err := s.InfoByHash(ctx, blockHash)" >> fix.patch
echo " 	if err != nil {" >> fix.patch
EOF

RUN git apply fix.patch

FROM --platform=$BUILDPLATFORM builder AS op-node-builder
RUN --mount=type=cache,target=/go/pkg/mod --mount=type=cache,target=/root/.cache/go-build cd op-node && make op-node  

FROM --platform=$TARGETPLATFORM $TARGET_BASE_IMAGE AS op-node-target
COPY --from=op-node-builder /app/op-node/bin/op-node /usr/local/bin/
CMD ["op-node"]
