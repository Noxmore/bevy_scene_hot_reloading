#!/bin/bash
set -e
set -x

{ echo; echo; echo "Run cargo fmt"; } 2> /dev/null
cargo fmt --check --all

{ echo; echo; echo "Run cargo clippy for default features"; } 2> /dev/null
cargo clippy --workspace --all-targets

{ echo; echo; echo "Run cargo clippy without default features"; } 2> /dev/null
cargo clippy --workspace --all-targets --no-default-features

{ echo; echo; echo "Run cargo doc with default features"; } 2> /dev/null
cargo doc --no-deps --workspace
