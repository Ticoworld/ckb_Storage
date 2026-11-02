# WO01 — Consensus boundaries

## Evidence and version boundary

Research date: 2026-11-02 UTC. CKB implementation:
`a4aa1286a45bc1d42c30f6698dc51521c3749b40` (package 0.210.0).
Release `v0.210.0` resolves to `2592ddf0502cd4adfe886db893cccc866db3c60f`.
The eight inspected capacity, resolution, script, DAO and mainnet-configuration
files have an empty diff between release and researched revision; see
[`release-comparison.json`](../evidence/release-comparison.json).
This establishes released implementation provenance, not the version currently
run by every mainnet node. No remote mainnet RPC or deployed contract was exercised.

[The source index](WO01_SOURCES.md) provides immutable links, RFC statuses and
exact revisions. [`source-files.json`](../evidence/source-files.json) adds file
SHA-256 hashes. Registry crate versions and checksums are in both Cargo lockfiles.
The executed system-script binaries come from **ckb-system-scripts 0.5.4**,
which the pinned CKB code uses; the separate system-scripts Git checkout is
reference source and is version 0.6.0. They must not be silently equated.

## Exact occupied-capacity rule

`util/gen-types/src/extension/capacity.rs` implements:

```
occupied_bytes = 8 + (32 + 1 + lock.args.len)
                 + optional(32 + 1 + type.args.len) + data.len
minimum_capacity_shannons = occupied_bytes * 100_000_000
```

`verification/src/transaction_verifier.rs::CapacityVerifier::verify` checks every
output against this amount and ordinarily checks inputs capacity >= outputs
capacity. DAO withdrawals and cellbase have dedicated accounting paths; they
are not arbitrary ways to mint capacity. A user script cannot waive this check.
Molecule offsets, length prefixes, outpoints, witnesses and historical blocks
are not all charged as occupied bytes by this formula. Do not confuse occupied
state with actual disk usage or serialized transaction bytes.

Measured with the real verifier, every untyped shape at its exact minimum passed;
one shannon below it failed **at capacity verification**:

| Structure | Occupied CKB | Security qualification |
|---|---:|---|
| Empty args, no type/data | 41 | A layout, not proof of ownership |
| 20-byte args | 61 | Standard sighash / multisig / base ACP layout |
| 22-byte args | 63 | Base Omnilock layout from RFC; actual Omnilock not executed |
| 32-byte args | 73 | Experimental two-key commitment |
| 64-byte args | 105 | Two explicit 32-byte keys, layout only |
| Empty args + 32-byte data root | 73 | Shared pool state, before claim backing |
| 20-byte lock args + type args 32 + data 16 | 142 | Official occupied-capacity calculation; no type contract executed |

The 41 CKB result does not show a general secure 41 CKB account with free
supporting infrastructure. The embedded-key experiment supplies actual authority
at that size by moving a key into a 44,616-byte executable.

## Transactions, groups and access

Current schema: `util/gen-types/schemas/blockchain.mol`. Inputs reference old
outpoints with a `since`; outputs contain capacity, a mandatory lock, optional
type, and associated output data. Cell dependencies supply code/data; header
dependencies supply selected headers, not a general lookup into the whole chain.
Witnesses are transaction-carried proof material. Raw transaction hashes exclude
witnesses. A lock must deliberately bind its signatures to the required fields;
“put it in a witness” does not authenticate it or make it persistent.

`script/src/types.rs` groups input locks by script hash and types from relevant
inputs/outputs. The same code with different arguments can form different groups.
The same lock/type code used in different roles is not automatically one group.
Input lock execution authorizes consumption. A lock appearing only on an output
is not executed merely because that output is created. Type scripts can enforce
creation and continuation when correctly attached. This is why our pool's plain
lock cannot certify initial root solvency: anybody can create a cell bearing it.

VM syscalls expose transaction-associated inputs, outputs, deps, witnesses,
headers and script-group views (RFCs 0009, 0034, 0046 and 0050). They do not expose
an oracle for global latest account state, Internet requests or guaranteed data
retrieval. Scripts can require an authenticated live configuration dependency;
that moves persistent state to a shared registry rather than eliminating it.
The resolver must reject spent dependencies; selecting a historical configuration
is not safe merely because its proof is cryptographically valid.

CKB-VM is not a cryptographic opcode whitelist: pure software Ed25519 executed
successfully here. Its cost is measured in VM cycles. The compiler target's
atomic instructions are **not** supported. The initial InvalidInstruction
(opcode 0x2f) was corrected using the SDK's documented `-C passes=lower-atomic`;
this is a compiler configuration fix, not a consensus change.

## Lifecycle, replay and retirement

Cells are immutable and consumed once. `resolve_transaction` rejects dead or
duplicate inputs; real resolver tests cover replay and competing withdrawals.
An in-memory resolver is used, not a hand-written stand-in double-spend rule.
A resolved script test alone does not establish a cell is still live on a chain.

Consuming a cell can release its occupied-state requirement. Capacity must still
land in valid outputs or become a transaction fee; it is not a separate refund
minted on closure. The historical transaction remains committed by the chain.
No unqualified guarantee of permanent archival service follows from a live root.

There is no consensus requirement for one cell per person, a wallet registration
cell, or a separate recovery-policy cell. Nor is there a consensus-native global
revocation of all future outputs to an old script. Our late-deposit test confirms
that spending/rotating one cell leaves a later old-lock deposit spendable by the
old key. Eliminating this needs sender migration, predetermined expiry, or an
authenticated current policy that such deposits already opted into.

## Economics and what requires a fork

RFC 0015 deliberately ties global-state rights to native-token capacity. The
one-CKB-per-byte unit is a consensus/economic design, not an information-theoretic
constant. Redefining this conversion or introducing independently transferable
protocol storage credits requires consensus rules, accounting and migration.
A smaller script or off-chain commitment is compatible with today's rules.

DAO principal is not a free loan of occupied state. `util/dao/src/lib.rs` computes
interest-bearing counted capacity as output capacity **minus occupied capacity**,
then applies the DAO accumulated-rate ratio and adds occupied capacity back.
Actual DAO use also has a type script, header dependencies and withdrawal timing.
It was source-inspected, not executed in this harness.

RFC metadata is not activation evidence: 0002 and 0015 are Final; **0019 is
Withdrawn**, 0022 is Draft, and 0026 is Proposal in the pinned tree. Current
implementation and mainnet genesis/configuration provide the operational cross-check.
Later VM/hardfork RFCs describe activation machinery; do not infer that an
unactivated future proposal already changes occupied-capacity accounting.

## Validation scope

Actual noncontextual, capacity and script verifiers; actual cell resolver;
mainnet-consensus defaults at a post-VM2 epoch. No PoW/header chain, block
construction, maturity/since suite, transaction-pool admission, reorg test,
network censorship experiment or mainnet deployment. These limitations prevent
calling the research contracts production-ready or a complete devnet result.
