# Reproduce WO02

Base is WO01 `ac79f83`; all original WO01 evidence and code remain unchanged.
New protocol code and outputs live exclusively under `experiments/wo02`,
`evidence/wo02`, and `docs/wo02`.

Prerequisites: Linux x86_64, Git, GCC/make, Python 3.12, Rust 1.95.0 with
`riscv64imac-unknown-none-elf`. Python verifier versions are pinned in
`requirements.txt` and were already installed on the execution machine.

```sh
# Install once into a local venv if your machine does not already provide them:
python3 -m venv /tmp/wo02-venv
. /tmp/wo02-venv/bin/activate
python3 -m pip install -r experiments/wo02/requirements.txt
rustup toolchain install 1.95.0 --profile minimal
rustup target add riscv64imac-unknown-none-elf --toolchain 1.95.0
bash experiments/wo02/run.sh > evidence/wo02/validator-run.txt 2>&1
python3 experiments/wo02/check_evidence.py
```

First fetches use the inherited pinned source manifest and frozen Cargo lockfiles;
no WO01 tests or generated outputs are rewritten. `CARGO_TARGET_DIR` defaults to
`/workspace/wo02-target`; set `WO02_TARGET_DIR` for another writable cache.
Existing toolchain activation on this machine:

```sh
export CARGO_HOME=/workspace/wo01-tools/cargo
export RUSTUP_HOME=/workspace/wo01-tools/rustup
export PATH="$CARGO_HOME/bin:$PATH"
```

The RISC-V contract needs the documented `-C passes=lower-atomic` flag. The host
uses actual CKB capacity/noncontextual/script validators and resolver at the same
pinned version as WO01. Code/data cells and chain anchors are in-memory fixtures;
this is not a running node, live mempool, deployment, light-client implementation,
security audit or availability guarantee.

`bin/anyone_can_pay` comes from the official Rust SDK at the exact commit recorded
in `evidence/wo02/sources.json`. Its CKB data hash matches the independently pinned
ACP source's expected `cd69…31e6`; it is not a custom ACP approximation. The binary
is retained so reproduction does not depend on an unpinned download. Its
underlying MIT source license is retained in `bin/ACP_LICENSE.txt`.

`proof_client.py` consumes only retained local user state and a serialized public
archive. It decodes actual Molecule transactions, checks outpoint continuity,
claim proofs/sums/signatures, successor state and an independently authenticated
expected live outpoint. The separate process has no operator tree/secret API.
The host then uses only its result, the current cell, recipient and user's own
key to build/sign a transaction and run actual CKB validators. Anchors are supplied
by this isolated verified test ledger, not a production network light client.

No node or credentials are needed after dependencies are cached. All keys are
public deterministic fixtures. No real funds. Work in the existing isolated
checkout; do not create worktrees or modify other projects.
