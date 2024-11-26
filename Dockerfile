# Use an official Rust image as the base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/ckks-engine

# Install nightly Rust toolchain
RUN rustup install nightly
RUN rustup default nightly

# Copy the rest of the project files
COPY . /usr/src/ckks-engine

# Install `cargo-fuzz` and dependencies
RUN cargo install cargo-fuzz

# Build the project using Cargo
RUN cargo build

# The default command will be to keep the container running
CMD ["tail", "-f", "/dev/null"]
