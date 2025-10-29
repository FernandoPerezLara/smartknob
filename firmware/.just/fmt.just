# List formatting commands
_default:
  @just --list fmt

# Format all code
apply:
  @cargo +nightly fmt --all

# Check code formatting without modifying files
check:
  @cargo +nightly fmt --all -- --check
