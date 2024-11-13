# This Dockerfile builds all the dependencies needed by the smart-contracts, excluding Go and Python.

FROM --platform=linux/amd64 us-docker.pkg.dev/oplabs-tools-artifacts/images/ci-builder:latest as foundry

FROM --platform=linux/amd64 debian:bookworm-20240812-slim as base

# Base: install deps
RUN apt-get update && apt-get install -y \
  curl \
  jq \
  ca-certificates \
  git \
  make \
  bash \
  --no-install-recommends

COPY --from=foundry /usr/local/bin/just /usr/local/bin/just
COPY --from=foundry /usr/local/bin/forge /usr/local/bin/forge
COPY --from=foundry /usr/local/bin/cast /usr/local/bin/cast

WORKDIR /opt/optimism

COPY ./versions.json ./versions.json
COPY ./packages ./packages
RUN git init
COPY .gitmodules ./.gitmodules

RUN git submodule update --init --recursive \
    && cd packages/contracts-bedrock \
    && just build 

FROM --platform=linux/amd64 debian:bookworm-20240812-slim as contracts-bedrock

RUN apt-get update && apt-get install -y \
  curl \
  jq \
  ca-certificates \
  git \
  make \
  bash \
  --no-install-recommends

COPY /ops/docker/oplabs.crt /usr/local/share/ca-certificates/oplabs.crt

RUN chmod 644 /usr/local/share/ca-certificates/oplabs.crt \
  && update-ca-certificates

COPY --from=foundry /usr/local/bin/just /usr/local/bin/just
COPY --from=foundry /usr/local/bin/forge /usr/local/bin/forge
COPY --from=foundry /usr/local/bin/cast /usr/local/bin/cast
COPY --from=foundry /usr/local/bin/svm /usr/local/bin/svm

RUN svm install 0.8.25 && \
  svm install 0.8.15 && \
  svm install 0.8.19 && \
  svm install 0.8.26

# Not to be confused with OP, this is a OnePassword CLI tool.
COPY --from=1password/op:2 /usr/local/bin/op /usr/local/bin/op

RUN mkdir -p /opt/optimism/packages/contracts-bedrock

COPY --from=base /opt/optimism/packages/contracts-bedrock /opt/optimism/packages/contracts-bedrock
COPY --from=base /opt/optimism/versions.json /opt/optimism/versions.json

WORKDIR /opt/optimism/packages/contracts-bedrock

# TODO: 
CMD ["echo", "Override this command to use this image."]