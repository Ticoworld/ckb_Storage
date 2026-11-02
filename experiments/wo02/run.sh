#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
# Use the pinned WO01 source checkouts; this does not execute or overwrite WO01 results.
python3 experiments/fetch_sources.py
python3 - <<'PY'
from importlib.metadata import version
for name, expected in [('cryptography','50.0.0'),('cffi','2.1.1'),('pycparser','3.0')]:
    assert version(name)==expected, f'Install experiments/wo02/requirements.txt: {name}'
PY
WO02_CACHE="${WO02_TARGET_DIR:-/workspace/wo02-target}"
RUSTFLAGS='-C passes=lower-atomic' CARGO_TARGET_DIR="$WO02_CACHE/contracts" cargo build --manifest-path experiments/wo02/contracts/Cargo.toml --target riscv64imac-unknown-none-elf --release --locked -j "${WO02_JOBS:-3}"
cp "$WO02_CACHE/contracts/riscv64imac-unknown-none-elf/release/wo02-contract" experiments/wo02/bin/sum_type
CARGO_TARGET_DIR="$WO02_CACHE/validator" cargo run --manifest-path experiments/wo02/validator/Cargo.toml --locked -j "${WO02_JOBS:-3}"
python3 experiments/wo02/economics.py
