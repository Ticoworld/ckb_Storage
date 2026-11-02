#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# Install Rust 1.95.0 + riscv64imac target first; see experiments/README.md.
python3 experiments/fetch_sources.py
mkdir -p experiments/bin
for mode in policy pool embedded; do
  RUSTFLAGS='-C passes=lower-atomic' cargo build --manifest-path experiments/contracts/Cargo.toml --target riscv64imac-unknown-none-elf --release --features "$mode" --locked -j "${WO01_JOBS:-3}"
  cp experiments/contracts/target/riscv64imac-unknown-none-elf/release/wo01-contract "experiments/bin/$mode"
done
cargo run --manifest-path experiments/validator/Cargo.toml --locked -j "${WO01_JOBS:-3}"
