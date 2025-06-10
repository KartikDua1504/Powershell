set -e # Exit on failure

cargo build --release --target-dir=/tmp/codecrafters-build-shell-rust --manifest-path Cargo.toml
