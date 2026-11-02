# WO01 — Break CKB storage assumptions

**Result: no architectural breakthrough is supported yet.** This repository
contains primary-source research and executable tests of recovery without an
account cell, shared authenticated CKB claims, and empty-argument ownership.

**72 real-validator/resolver checks pass**, including expected attack rejections,
plus 240 host-side proof-update comparisons. Existing 1-of-2 multisig already
supports the tested independent recovery model in a 61 CKB asset cell. A shared
73-byte root supports conditional independent exits and public proof refresh,
but the prototype lacks admission solvency checks and dynamic payment/recovery
semantics. Small pooled claims do not automatically fund fresh L1 exits.

## Research index

- [Consensus boundaries and RFC status](docs/WO01_CONSENSUS_BOUNDARIES.md)
- [First-principles questions](docs/WO01_FIRST_PRINCIPLES_QUESTIONS.md)
- [Existing approaches and novelty comparison](docs/WO01_EXISTING_APPROACHES.md)
- [24-hypothesis register](docs/WO01_HYPOTHESIS_REGISTER.md)
- [Capacity, capital and price economics](docs/WO01_CAPACITY_ECONOMICS.md)
- [Experiment results and adversarial coverage](docs/WO01_EXPERIMENT_RESULTS.md)
- [Breakthrough decision and next decisive experiment](docs/WO01_BREAKTHROUGH_DECISION.md)
- [Pinned primary-source index](docs/WO01_SOURCES.md)

## Reproduce

See [exact prerequisites and scope](experiments/README.md). With Rust 1.95.0,
its `riscv64imac-unknown-none-elf` target, Git and GCC/make installed:

```sh
bash experiments/run.sh > evidence/validator-run.txt 2>&1
python3 experiments/check_evidence.py
```

Source checkouts live in `../wo01-sources` at exact commits from
[evidence/sources.json](evidence/sources.json). Frozen Cargo lockfiles pin crates.
No node, production credentials or real funds are required. Three RISC-V
contract binaries are retained in [experiments/bin](experiments/bin).

## Evidence

- [Raw reproduction output](evidence/validator-run.txt)
- [Per-case accepts, rejections, errors, cycles and transaction sizes](evidence/validator-results.json)
- [Generated population-capacity tables](evidence/CAPACITY_TABLES.md)
- [120 price/capital scenarios (CSV)](evidence/capacity-model.csv)
- [Illustrative fee scenarios](evidence/fee-scenarios.json)
- [Released implementation comparison](evidence/release-comparison.json)
- [Source file hashes](evidence/source-files.json)

The actual CKB noncontextual/capacity/script validators and cell resolver are
used with an in-memory provider. This is not a live devnet, mempool benchmark,
mainnet deployment or security audit. Million-user totals are extrapolations;
large-tree timing rows execute one signer against synthetic sibling commitments.
Research contracts have public test keys and explicit limitations; use no real
funds. JoyID's specific 150 CKB upgrade and QuotaCell's 600 CKB policy operation
remain unverified; the inspected CoTA SDK separately allocates 150 CKB.

Work is isolated on **`wo01-storage-research`**, with meaningful milestones
committed and pushed. The unrelated initial `hello.txt` is retained.
