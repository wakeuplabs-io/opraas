# Use Alpine as the base image
FROM rust:latest

# Install Foundry (anvil) using Cargo from GitHub
RUN cargo install --git https://github.com/foundry-rs/foundry anvil --locked --force

# Verify installation
RUN anvil --version

# Set the default command
CMD ["bash"]
