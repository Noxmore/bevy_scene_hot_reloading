echo Run cargo fmt
cargo fmt --all --check || exit /b 1

echo Run cargo clippy for default features
cargo clippy --workspace --all-targets --exclude physics || exit /b 1

echo Run cargo clippy without default features
cargo clippy --workspace --all-targets --no-default-features --exclude physics || exit /b 1

echo Run cargo doc with default features
cargo doc --no-deps --workspace || exit /b 1
