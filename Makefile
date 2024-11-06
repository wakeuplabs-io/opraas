

ifeq (run,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "run"
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(RUN_ARGS):;@:)
endif

APPLE_TARGET=x86_64-apple-darwin
WINDOW_TARGET=x86_64-pc-windows-gnu
LINUX_TARGET=x86_64-unknown-linux-musl

run:
	@cargo run -q -p opraas_cli -- $(RUN_ARGS)

format:
	cargo fmt --all

lint:
	cargo clippy --fix

build-windows:
	cargo build --target=${WINDOW_TARGET} --release

build-linux:
	cargo build --target=${LINUX_TARGET} --release

build-apple:
	cargo build --target=${APPLE_TARGET} --release

check-commit:
	@if ! git diff-index --quiet HEAD --; then \
		echo "ERROR: There are uncommitted changes. Please commit them before proceeding."; \
		exit 1; \
	fi;

bump-cli-rust-version: check-commit
	@echo "Bumping Rust version to $(VERSION)"
	# Update the version in Cargo.toml
	@sed -i.bak 's/^version = ".*"/version = "$(VERSION)"/' ./opraas_cli/Cargo.toml

bump-cli-npm-version: check-commit
	@echo "Bumping npm version to $(VERSION)"
	# Update the version in package.json
	@cd npm && npm version $(VERSION) --no-git-tag-version

bump-versions: bump-cli-rust-version bump-cli-npm-version

release-cli: bump-versions
	@echo "Creating Git tag $(VERSION)"
	@git tag -a $(VERSION) -m "Release version $(VERSION)"
	@echo "Ready, review changes and push tag with git push origin $(VERSION)"


