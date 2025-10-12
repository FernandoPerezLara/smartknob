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

# Install Just and ldproxy
RUN cargo install just
RUN cargo install ldproxy

# Set working directory
WORKDIR /app

# Copy Rust configuration files
COPY rust-toolchain.toml rustfmt.toml ./
COPY justfile ./
COPY .just/ ./.just/

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN mkdir -p build && echo "fn main() {}" > build/main.rs
RUN cargo fetch
RUN rm -rf src/ build/

# Copy project files
COPY src/ ./src/
COPY build/ ./build/
COPY assets/ ./assets/

# Default command
CMD ["just", "check"]
