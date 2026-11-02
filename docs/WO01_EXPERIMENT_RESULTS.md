# WO01 — Experiment results

## Outcome and reproducibility

**72 expected outcomes passed**, including legitimate accepts and deliberate
attack/invalid-transaction rejections. An additional **240** host-side comparisons
check public proof refresh at all ordered distinct index pairs of a 16-leaf tree.
These are separate from the VM/resolver case count. No skipped/expected-failure
framework tests or zero-test success is being substituted for validation.

Commands and exact tool requirements: [experiments/README.md](../experiments/README.md).
Complete workflow:

```sh
bash experiments/run.sh > evidence/validator-run.txt 2>&1
```

Source code: [contracts](../experiments/contracts/src/main.rs),
[validator and transaction builder](../experiments/validator/src/main.rs),
[model](../experiments/model.py). Outputs:
[raw run](../evidence/validator-run.txt),
[machine-readable cases](../evidence/validator-results.json),
[capacity tables](../evidence/CAPACITY_TABLES.md).
Both Cargo lockfiles and the source manifest pin dependencies.

The actual CKB noncontextual, capacity and script verifiers run against real
RISC-V ELFs. `resolve_transaction` uses the real resolution implementation with
an in-memory provider. System locks use VM0; custom contracts use VM2 at epoch
100,000 under pinned consensus defaults. This is an isolated verification
environment, **not** a running devnet or a verified mainnet deployment. Header
chain/PoW, maturity/since, reorgs and mempool fee admission are outside its scope.

Input/dependency outpoints are synthetic but explicitly resolved. Code cells
are fixtures, not published contracts. All signing keys are deterministic public
test fixtures, including the RFC8032 embedded-key test seed. Do not use them for
real funds. Some layout and retirement tests use the CKB always-success fixture;
its small size is **not** offered as secure ownership. Real competitive owner
and recovery tests use the bundled secp/multisig code; pool payout outputs use
the standard 61-byte lock layout, which need not run at output creation.

## E0: occupied-capacity boundaries and reusable baselines

For untyped 0/20/22/32/64-byte lock arguments and the 32-byte data-root layout,
exact minimum passes and minimum-minus-one-shannon fails in CapacityVerifier.
A typed 142-byte layout is calculated by the official occupied-capacity method;
its type script was not deployed/executed. Base Omnilock and ACP are layout
measurements, not executions of their contracts.

Existing system sighash signing, 1-of-2 multisig owner signing and independent
recovery signing pass. Unrelated signers fail. The multisig config is revealed
in WitnessArgs, while its 20-byte hash remains in the asset lock. This directly
falsifies a universal need for a separate recovery-registration cell.

| Valid spend | Cycles in retained run | Packed transaction bytes |
|---|---:|---:|
| Existing standard sighash | 1,720,212 | 388 |
| Existing 1-of-2 multisig, owner | 1,728,521 | 432 |
| Existing 1-of-2 multisig, recovery | 1,734,665 | 432 |
| Custom policy, owner | 1,845,650 | 375 |
| Custom policy, recovery | 1,814,718 | 375 |
| Embedded-key owner | 1,836,954 | 310 |
| Pool independent exit, depth 2 | 1,909,310 | 560 |
| Pool exit after local public-proof refresh | 1,912,690 | 560 |

These are concrete fixture transactions, **not** a controlled apples-to-apples
performance ranking: signature algorithm, policy, recipient, witness format and
dependencies differ. Each transaction declares only its required code dependency
and the standard locks also declare secp data. ELF/code occupied capacity is
counted separately. Cryptographic signature execution can vary with the signed
message; changing fixture outpoints changes cycle counts.

## E1: recovery commitment inside an asset lock

Args are `CKB_BLAKE2b(owner_ed25519_key || recovery_ed25519_key)` (32 bytes).
Witness is `path_byte || owner_key || recovery_key || signature` (129 bytes).
The script verifies the policy commitment, chooses the authorized key, and
strictly verifies its Ed25519 signature of the raw transaction hash. Raw inputs
and outputs, including the old outpoint and new lock, are therefore bound.

Owner/recovery spends pass. Forged policy, signature, wrong branch and changed
outputs fail. Recovery can place funds under a new committed key pair; an old
policy cannot spend that successor, and the new owner can. Resolver replay of
the old input fails.

**Adverse success:** a new cell deposited to the old static lock remains
spendable by the old owner after another cell was rotated. This is a
counterexample, not a recovery guarantee. The custom policy and existing 1-of-2
both also grant either credential immediate spending authority: compromise of
either is not prevented. No delay, threshold recovery, credential recovery after
all keys are lost, application spending limit or universal revocation is claimed.

Occupied capacity is 73 CKB versus the existing multisig's 61 CKB for this narrow
authority model. Code adds 52,621 CKB at the measured deployment layout.
**Decision: EXISTING SOLUTION; custom encoding is not competitive.**

## E2: independently signed claims in one root cell

A pool uses empty args, no type, and a 32-byte Merkle root. Leaf hash is
`H(0 || public_key[32] || amount_u64_le)`; internal node is
`H(1 || left[32] || right[32])`; spent leaf is `H(2)`. Domain separation avoids
confusing leaf/internal/spent encodings. Witness is
`index_u32 || amount_u64 || pubkey[32] || depth_u8 || siblings || signature[64]`.
Maximum depth is 20. Proof/signature authenticate the claim. The script changes
exactly that leaf to spent, preserves the successor lock, enforces capacity
accounting, and creates one selected payout minus a hardcoded 1,000-shannon fee.
The signature binds the payout and successor transaction. Typed assets and extra
inputs are deliberately unsupported; this is a CKB-claims experiment.

Four different keys can independently exit. Old-input replay/concurrent sibling
spends fail in the real resolver; reused claims and stale proofs fail against
the successor root. Reauthorized excessive withdrawals and incorrect successor
roots fail even when their transaction signatures are correct. Missing/truncated
proofs, forged siblings/index/signatures, unsupported typed inputs and duplicate
inputs are rejected at the reported stages.

### Stronger variant after stale-proof failure

For users at indices i and j, an exit by j changes exactly one sibling subtree
in i's cached proof. At level `floor(log2(i XOR j))`, recompute j's changed
subtree from its public exit proof below that level and the spent marker;
replace that sibling in i's proof. No operator signing key or full account
database is necessary for this fixed withdrawal-only transition.

`refresh_proof` implements this update. `assemble_exit_from_local_proof` constructs
the complete successor transaction using **only** current chain cell metadata,
user key/amount/index, cached proof and payout script. It does not receive the
full leaf tree. The refreshed exit passes actual VM validation. The final user
also refreshes across all preceding exits before closure. Full-tree calculations
are retained separately as test oracles, not as inputs to this local builder.
The 240 host checks cover every pair of distinct indices in a synthetic 16-leaf
tree; they are not a full formal proof or million-user DA test.

This preserves conditional independent exit after the operator disappears,
**provided** the user obtained an authentic initial proof, the initial pool is
solvent, code/data are available, and the user receives every relevant public
transition. It does not establish recovery after all initial proofs and witness
archives are lost, guarantee inclusion against miners, or support arbitrary
payments/deposits/key rotations whose data deltas were never published.

### Capital/creation counterexamples

Four 100 CKB claims close from both (a) 473 CKB including a donated reserve and
(b) **400 CKB with no additional reserve**. In (a), the final claimant receives
the remaining reserve by an explicit prototype rule. In (b), the remaining
claims themselves underwrite root occupancy until the final root is retired.
There is no compulsory additive 73 CKB beyond native principal in that case.

A real creation transaction can also place the same 400 CKB claims root in a
**173 CKB** pool. Creation succeeds: an output-only lock does not run an
admission check. The first 100 CKB exit succeeds, but a subsequent 100 CKB exit
cannot be funded. The root proves membership, not total liabilities. A consumer
must verify admission/funding independently, or a future type-enforced sum-tree
protocol must enforce it. This is a decisive blocker to advertising the present
pool as a generally secure deposit system.

Two 1 CKB claims in a funded 75 CKB root cannot pay an ordinary fresh standard
recipient cell: capacity verification rejects the payout before scripts. A
recipient with existing ACP state could change this result in a different
multi-input protocol; that is not implemented or hidden in this result.

### Scaling measurements

| Logical users | Tree depth | One witness bytes | One packed tx bytes | One exit cycles |
|---:|---:|---:|---:|---:|
| 1 | 0 | 109 | 375 | 1,839,405 |
| 10 | 4 | 237 | 624 | 1,960,696 |
| 1,000 | 10 | 429 | 816 | 2,094,794 |
| 1,000,000 | 20 | 749 | 1,136 | 2,278,569 |

Each row executes **one real signer and proof** with opaque synthetic sibling
subtree roots. Ten means depth for a tree padded to 16; a million is padded to
2^20. It is not an independently populated/solvency-checked million-user tree,
a million-exit simulation or a network throughput test. User-count capital
results are labeled extrapolations in the economics document.

**Decision: EXISTING SOLUTION** for the demonstrated fixed-claims/shared-UTXO
mechanism. A dynamic, solvent, recoverable, competitively superior system is
**RESEARCH OPEN**, not a proved breakthrough.

## E3: empty args, key in executable code

A constant public key in the RISC-V ELF verifies strict Ed25519 signatures with
empty args. The owner succeeds; an unrelated key fails. Occupied live-cell
minimum is **41 CKB**. Exiting a cell exactly at that minimum to another 41 CKB
cell while subtracting a fee fails capacity validation; fees need working capital
or additional inputs in a protocol that permits them.

The ELF is 44,616 bytes; at a 20-byte deployment lock its code cell occupies
44,677 CKB. New independent keys need distinct bound code (or other identity
state). Reusing the same constant simply reuses ownership. There is no general
independent-user saving once supporting code is counted.
**Decision: REJECTED as a per-user capital breakthrough.** It is a valid
counterexample to “every secure lock needs nonempty args.”

## Adversarial coverage and remaining gaps

| Requirement | Evidence | Limit |
|---|---|---|
| Unauthorized claims / forged witnesses | Real signature, key, policy, path, sibling/index negatives | No large cryptographic fuzzing or audit |
| Replay / repeated withdrawals | Real resolver dead/duplicate-input checks and successor spent-leaf rejection | No fork/reorg simulation |
| Excessive withdrawals | Correctly signed malicious payout/root transitions rejected | Admission solvency not enforced; creation counterexample preserved |
| Concurrent transitions | Individually valid exits conflict in resolver | No network/mempool fairness or batch implementation |
| Malicious/missing state provider | Wrong and missing proofs fail; local refresh works from public exits | Initial proof loss or total history withholding not solved |
| Operator disappearance | No operator signature in withdrawal path; local-only exit builder | Chain data/code access and inclusion remain assumptions |
| Credential loss/compromise | Independent recovery positive, unrelated/old-successor credentials negative | One-of-two permits either key theft; all-key loss unrecoverable |
| Revocation / late deposits | Rotated successor rejects old policy; old-lock late deposit accepts old key | No global registry/delay/revocation implementation |
| Forced/delayed exits | User constructs fixed-claim exit without operator | No time-bound inclusion proof, channel dispute or since test |
| Hidden capacity | ELF sizes, secp data, principal and existing infrastructure counted | Competitor full deployments and DA operating costs not benchmarked |
| Simultaneous large exits | One-signature depth-20 execution plus resource/capital extrapolation | No million-account workload or live contention benchmark |

## Failures retained and corrected

1. Initial contract build incorrectly chained the blake2b-ref `update` method,
   which returns unit. Source/API correction separates update/finalize;
   [`initial-contract-build-failure.txt`](../evidence/initial-contract-build-failure.txt)
   is preserved. This was a build error, not an architecture rejection.
2. First VM run hit unsupported atomic opcode 0x2f with the default Rust RISC-V
   target. The pinned SDK documents `-C passes=lower-atomic` for Rust 1.95;
   rebuilding all three contracts fixes it. The initial VM output is preserved.
3. The plain-root insolvency, stale-proof, concurrent-exit and late-deposit
   behaviors remain permanent regression evidence. Only stale proofs in the
   named fixed-claims protocol have a tested stronger availability variant.

No TLS/signature/checksum verification was disabled. No application outside this
research repository was modified. No on-chain funds were used.
