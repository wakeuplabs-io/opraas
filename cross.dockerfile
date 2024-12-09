FROM ghcr.io/cargo-lambda/cargo-lambda:latest

RUN apt-get update && \
    apt-get install -y libssl-dev pkg-config

WORKDIR /usr/src/app

COPY . .

RUN cargo lambda build --release --package opraas_server
