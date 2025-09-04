##########################################
# Rust Environment
##########################################
FROM rust:1.89

# Install necessary components and targets
RUN rustup component add rustfmt clippy rust-src --toolchain nightly
RUN rustup target add riscv32imac-unknown-none-elf
RUN rustup toolchain install nightly
RUN rustup component add rustfmt clippy --toolchain nightly
RUN rustup target add riscv32imac-unknown-none-elf --toolchain nightly

# Set working directory
WORKDIR /app

# Copy Rust configuration files
COPY rust-toolchain.toml rustfmt.toml build.rs ./

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN rm -rf src/

# Copy source code
COPY src/ ./src/

# Default command
CMD ["cargo", "+nightly", "check", "--release"]
