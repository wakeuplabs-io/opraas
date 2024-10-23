

ifeq (run,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "run"
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(RUN_ARGS):;@:)
endif

RELEASE_VERSION=0.0.1
APPLE_TARGET=x86_64-apple-darwin
WINDOW_TARGET=x86_64-pc-windows-gnu
LINUX_TARGET=x86_64-unknown-linux-musl

run:
	@cargo run -p opraas_cli -- $(RUN_ARGS)

format:
	cargo fmt --all

lint:
	cargo clippy --fix

release-windows:
	cargo build --target=${WINDOW_TARGET} --release
	mkdir -p releases/${RELEASE_VERSION}/${WINDOW_TARGET}/dist
	cp target/${WINDOW_TARGET}/release/opraas_cli releases/${RELEASE_VERSION}/${WINDOW_TARGET}/dist/opraas_cli
	tar -czvf releases/${RELEASE_VERSION}/${WINDOW_TARGET}/opraas-v${RELEASE_VERSION}-${WINDOW_TARGET}.tar.gz release/${WINDOW_TARGET}/dist

release-linux:
	cargo build --target=${LINUX_TARGET} --release
	mkdir -p releases/${RELEASE_VERSION}/${LINUX_TARGET}/dist
	cp target/${LINUX_TARGET}/release/opraas_cli releases/${RELEASE_VERSION}/${LINUX_TARGET}/dist/opraas_cli
	tar -czvf releases/${RELEASE_VERSION}/${LINUX_TARGET}/opraas-v${RELEASE_VERSION}-${LINUX_TARGET}.tar.gz release/${LINUX_TARGET}/dist

release-apple:
	cargo build --target=${APPLE_TARGET} --release
	mkdir -p releases/${RELEASE_VERSION}/${APPLE_TARGET}/dist
	cp target/${APPLE_TARGET}/release/opraas_cli releases/${RELEASE_VERSION}/${APPLE_TARGET}/dist/opraas_cli
	tar -czvf releases/${RELEASE_VERSION}/${APPLE_TARGET}/opraas-v${RELEASE_VERSION}-${APPLE_TARGET}.tar.gz release/${APPLE_TARGET}/dist


.PHONY: run
