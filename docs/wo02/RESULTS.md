# WO02 — Executable results

Base: `ac79f830c896b8b431de9ce869df0883ffc416d5`. Reproduction instructions are in
[`experiments/wo02/README.md`](../../experiments/wo02/README.md). No existing WO01
evidence, source or hypothesis document was rewritten or reclassified.

## Summary

- **142** real-validator/resolver cases passed their expected outcomes:
  **110 accepts, 32 rejections**.
- **24 complete withdrawal orders**, four independently keyed claims per order:
  96 actual validated exit transactions, with liabilities and reserve checked
  after every successor and full retirement checked at every final exit.
- **144 public proof refreshes** across these orders.
- Separate process reconstruction followed three serialized public exits and
  produced a proof used in a further **accepted** real-validator exit.
- Four additional archive attacks were rejected by that client. These are
  separate from the 142 core cases.
- A full-manifest admission with **256 actual independently derived keys** passed.
  This tests the deliberately bounded admission ceiling, not all 256 exits or
  a million-user system.

Raw output: [`validator-run.txt`](../../evidence/wo02/validator-run.txt).
Structured output: [`validator-results.json`](../../evidence/wo02/validator-results.json).
Public archive and retained/reconstructed local state:
[`operator-gone/`](../../evidence/wo02/operator-gone/).

## What the decisive experiment establishes

| Requirement | Actual evidence |
|---|---|
| Principal-underfunded roots cannot be admitted as solvent | 400 CKB of manifest claims with only 200 CKB output backing passes ordinary capacity accounting but is rejected by the **type script** |
| Reserve/funding invariant | One shannon below required sum+reserve rejected by type; this test concerns required reserve, not a claim that its monetary principal alone was underfunded |
| Correct root and liabilities | False root or separately falsified total rejected; exact public-manifest reconstruction required |
| Overflow safety | Sum of leaves overflow, liabilities+reserve overflow, and withdrawal proof-sum overflow rejected |
| Accepted transitions conserve remaining liabilities | Correctly signed successor-liability reduction, reserve theft, premature full closure and stripping the type rejected; 96 sequential exits across all four-claim permutations accept |
| Independent claim authority | Forged key, index, amount, sibling hash/sum, signature and truncated proof rejected |
| Replay and conflicts | Reused claim rejected on successor; old-input replay and competing valid exits conflict in actual resolver |
| Instance continuity | Creation ID mismatch and attempted recreation from a new funding outpoint rejected |
| Operator-loss reconstruction | Separate Python client consumes only local retained proof and public packed transactions; local builder creates accepted signed exit from reconstructed proof |
| Public-data adversary | Missing middle, forged membership witness, forged claim signature and wrong independently authenticated tip rejected by client |
| Tiny exit economics | 1 CKB fresh standard output fails capacity; existing **real ACP** recipient credit accepts and recipient independently spends it afterward |

These are representative adversarial cases plus finite exhaustive ordering for
four claims. They are not a formal proof, unrestricted protocol fuzzing, network
censorship test, block validation/reorg suite or production audit.

## Measured transaction shapes

| Valid transaction | Packed bytes | VM cycles | Actual fixture fee, shannons |
|---|---:|---:|---:|
| Admit four claims, 1/2/3/4 CKB | 696 | 1,804,297 | 10,000 |
| Pool 1 CKB gross claim into existing ACP | 853 | 1,973,987 | 10,000 |
| Direct ACP credit, same 1 CKB gross budget | 590 | 1,737,001 | 10,000 |
| Direct ACP credit of full 1 CKB, fee paid in addition | 590 | 1,753,513 | 10,000 |
| ACP owner spends credited cell to standard lock | 392 | 1,669,495 | 10,000 |
| Reconstructed final claim exit, including reserve release | 638 | 1,943,761 | 10,000 |
| Admit 256 actual one-CKB claims | 10,776 | 5,915,228 | 20,000 |

The same-gross-budget pool/direct-ACP comparison delivers exactly **0.9999 CKB**
net in both cases. It uses the same existing recipient, fee and funding-control
assumptions. Pool settlement is larger and slower in these concrete fixtures.
This is not a universal performance ranking: signatures depend on signed bytes,
transaction construction differs and no mempool/fee market was measured.

10,000 shannons = 0.0001 CKB. The larger admission uses 20,000 shannons so the
illustration does not silently assume a 10,000 fee covers a >10,000-byte
transaction at a hypothetical 1,000-shannon/kB floor. Actual mempool admission
was **not tested**. Negative capacity/admission probes intentionally use other
fee differences and were never broadcast; they are not paid operational fees.

## Dependencies, code and execution boundary

Actual CKB `NonContextualTransactionVerifier`, `CapacityVerifier`,
`TransactionScriptsVerifier` and `resolve_transaction` use in-memory cells and
post-VM2 epoch context at the same pinned CKB version as WO01. No live node,
mining, mainnet deployment or independent network archive was run.

| Supporting component | Bytes | Occupied CKB under a 20-byte deployment lock |
|---|---:|---:|
| New Merkle-sum type ELF | 64,896 | 64,957 |
| Pinned always-success lock ELF | 344 | 405 |
| Actual ACP ELF | 66,048 | 66,109 |
| Existing bundled standard sighash ELF | 52,048 | 52,109 |
| Shared secp data | 1,048,576 | 1,048,637 |

For the immutable burn-lock code fixtures, new pool code/lock capital totals
**65,322 CKB** (64,937 + 385), counted explicitly and irrecoverable. The
20-byte owner-lock alternative in the table costs **65,362 CKB** and would permit
its owner to remove code; that alternative cannot silently inherit the same
operator-failure guarantee. The existing
ACP/standard/secp infrastructure totals **1,166,855 CKB of occupied minima** for
the tested lifecycle; it is shared, not paid anew for every user. These are not
actual configured chain holdings or new deployments. The claimed funding and
147 CKB reserve are separate from these shared code cells.

The ACP binary is from the official SDK at the commit in `sources.json`; its
CKB data hash is **cd69ba816f7471e59110058aa37387c362ed9a240cd178f7bb1ecee386cb31e6**,
matching the expected hash in the already pinned ACP source's build script.
It is not custom code implementing “ACP-like” behavior.

The initial state/expected live outpoint passed to the reconstruction client must
be independently authenticated. In this test the anchor is supplied by the
validated fixture ledger. The client is a proof updater and transaction decoder,
not a production CKB light client. An untrusted archive's public bytes can be
checked; their permanent availability and canonical chain inclusion are distinct
requirements. Missing history prevents reconstruction rather than authorizing
a guessed exit. Public copies of the required artifacts are committed here.

## Failures retained rather than hidden

1. The first new contract build had a duplicated alloc import and missing Entity
   trait import. Correct imports fixed the build without weakening verification.
2. An initial wrong-lock negative changed occupied size, so capacity rejected it
   before the intended type check. It was corrected to a wrong code hash at the
   same empty-args layout, isolating the type rule.
3. The initial successor-replay negative reused old output amounts, causing an
   accounting overflow before membership validation. Outputs were rebalanced to
   the successor's funding; the reauthorized reused claim then fails membership.
4. The first client “signature attack” mutated a trailing unrelated witness field,
   not the claim signature. The corrected fixture re-encodes a valid Molecule
   transaction with a changed **claim signature**; it fails signature verification.
   This is not presented as a fixed cryptographic vulnerability in the client.

5. A substantive protocol edge case was then found: an admitted one-shannon
   positive claim could not pay the fixed exit fee. The accepting counterexample
   is preserved in `fee-dust-admission-counterexample.txt`. Admission now rejects
   all nonempty claims at or below 10,000 shannons. A 10,001-shannon admitted
   claim pays its fee and successfully credits **one net shannon** to real ACP.

Those initial logs remain in `evidence/wo02/initial-*.txt`. The pre-fee-guard
contract source and binary are also retained in Git at milestone `efd7457`.
No TLS, checksum, cryptographic signature verification or meaningful assertion
was disabled.

## Gate outcome

The admission and fixed-claim withdrawal mechanism works under its specified
failure model. It does **not** meet the competitive/economic advance threshold.
No general payments, recovery UI, multi-shard coordination or wallet was added.
The protocol remains a reviewable bounded experiment, with the recommendation
and economic limits in [DECISION.md](DECISION.md).
