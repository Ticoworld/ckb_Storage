# Reproduce WO01

Research contracts only. Never deposit real funds. Keys in the harness are public,
deterministic test fixtures. No production credentials, node or network required
once dependencies and pinned sources are cached.

Prerequisites: Linux x86_64, Git, TLS-enabled network access for first fetch,
GCC/make, Rust **1.95.0**, RISC-V target `riscv64imac-unknown-none-elf`.

```sh
rustup toolchain install 1.95.0 --profile minimal
rustup target add riscv64imac-unknown-none-elf --toolchain 1.95.0
bash experiments/run.sh > evidence/validator-run.txt 2>&1
```

Run from this checkout. `fetch_sources.py` verifies all exact revisions in
`evidence/sources.json`, placing reference sources in `../wo01-sources`.
It refuses to reset or overwrite an existing different or modified checkout.
`Cargo.lock` files pin registry checksums. CKB host crates come from the pinned
source checkout; contract ckb-std comes from crates.io 1.1.0, not the reference
SDK Git checkout. Research binaries are retained with hashes in results.

The SDK's documented `-C passes=lower-atomic` is mandatory for Rust 1.95:
CKB-VM does not implement the target's native A instructions. The initial failure
and supported correction are preserved in `evidence/`.

This uses actual CKB `NonContextualTransactionVerifier`, `CapacityVerifier`,
`TransactionScriptsVerifier` and `resolve_transaction`, with an in-memory cell
provider. VM2 custom scripts are exercised at epoch 100,000 under the pinned
consensus defaults; the existing system scripts use their original VM0.
No node, mining, live mempool, block acceptance, fork reorg, RPC or mainnet
submission is tested. Failed malicious transactions are expected rejections;
`test_passed` is distinct from `accepted` in the result JSON.

Local onboarding machine uses these tool locations (not required elsewhere):

```sh
export CARGO_HOME=/workspace/wo01-tools/cargo
export RUSTUP_HOME=/workspace/wo01-tools/rustup
export PATH="$CARGO_HOME/bin:$PATH"
```

Each task already has an isolated checkout. Do not create a worktree unless
explicitly requested. Execute commands only in this repository and reference
cache; do not modify other application projects.
