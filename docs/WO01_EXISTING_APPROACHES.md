# WO01 — Existing approaches and novelty check

All source revisions are in [WO01_SOURCES.md](WO01_SOURCES.md). “Inspected” below
means documentation/source inspection; only bundled sighash and multisig were
executed as competitive lock baselines. No competitor application end-to-end
benchmark or independent security audit is claimed. Storage proposals and
implementations are not assumed to be current deployments merely because a
repository exists.

| Approach | What it solves | Capital and control | Failure boundary / remaining problem |
|---|---|---|---|
| Standard secp256k1/blake160 sighash | Single-key ownership; no account registration | 61 CKB minimum per untyped cell; owner controls it; bundled code and 1 MiB secp data already deployed | Lost key loses authority; static lock has no global revocation; separate tiny cells require capacity |
| System multisig, 1-of-2 | Precommitted owner OR recovery key; rotation by spend | Same 61 CKB minimum, 20-byte policy commitment; shared genesis code/data | Either key can steal; no recovery delay or universal late-deposit protection; both keys lost means no recovery |
| Compact custom lock | Alternate identity/hash/signature and policy encoding | 41+args+data bytes, plus all code/dependency capacity; controlling party follows lock | Shortening binding can reduce security; embedded identity or registry can hide relocated cost |
| Anyone-Can-Pay | Adds value to a recipient's existing cell without recipient signature on the permitted deposit path | Base 20-byte args => 61 CKB; optional thresholds add bytes; recipient owns capital | Does not erase initial recipient capacity; contention and dust/DoS mitigation matter; not shared independent claims by itself |
| Omnilock | Multiple auth methods, administrator paths, ACP, timelocks and supply modes | Base args 21-byte auth + 1-byte flags => 63 CKB; optional admin Type ID adds 32 bytes and admin-list infrastructure | Admin may have broad authority; registry availability/updates and configuration determine security; not zero-state recovery |
| JoyID mechanisms | Passkey-oriented signing; main/subkey modes and social-recovery-related SDK requests | Asset cells plus account/CoTA extension machinery where selected; user authority and recovery policies depend on mode | Aggregator endpoints provide proof generation in inspected SDK; loss of provider and credential compromise require independent state reconstruction analysis |
| CoTA | Aggregates token/account facts in SMT commitments instead of one token cell each | Inspected SDK allocates **150 CKB per CoTA registration cell**, plus shared registry, code and asset/infrastructure costs | SDK registration still iterates per-lock CoTA cells; proof generation relies on aggregators backed by synced witnesses; token aggregation is already an existing design |
| Open Transactions / transaction co-build | Partial authorization and multi-party transaction assembly / intent composition | Inputs/outputs still obey capacity; fillers or owners supply capital; chosen lock defines signed constraints | Counterparty or filler disappearance can stop completion; partial-signing replay and overbroad scopes require care; does not by itself create cheaper persistent storage |
| Fiber / payment channels | Many off-chain payments per funding/settlement structure | Channel funding, balances/liquidity, reserves/commitments and code; channel parties control paths | Must preserve latest state and satisfy force-close/challenge/monitoring assumptions; mass closures use L1 resources; routing liquidity is not storage-free |
| Shared UTXO / Merkle claims | Many authenticated rights per live commitment | Shared root+backing+code; correct transitions should preserve independent claimant authorization | Serial outpoint, proofs, admission funding, recoverability and exit costs; WO01 is an instance of this existing family |
| Optimistic rollup (Godwoken reference) | Aggregated execution, custodied assets, challenge/finality machinery | Rollup/state, deposit, custodian and withdrawal cells plus code, backing and operator infrastructure | Sequencing, fraud proof/data availability and timeout routes must work after operator failure; source-described deposit/withdrawal cost is not root size alone |
| Validity rollup / validium families | Succinct validity checks and aggregated state | Root, verifier code, asset backing, proof generation, escape structures, DA infrastructure | A proof of correctness does not prove user data availability; validium committee disappearance may prevent exits; no implementation from this family executed here |
| RGB++ | Delegates CKB asset control to Bitcoin UTXO seals with SPV validation | CKB asset/type/data cells, Bitcoin UTXOs/fees, SPV client and timelock contracts | Cross-chain reorg/finality and proof retrieval; ownership changes representation, but the CKB cell is still present |
| Capacity leasing/sponsoring | Another party allocates capacity or funds UX | Lender/sponsor still supplies capital; agreements constrain reclaim/control | Can improve access and turnover; does not reduce total live capacity by changing payer |

## Specific primary-source findings

**Recovery comparison.** System multisig config is `[0, require_first_n,
threshold, pubkeys_count] || blake160(pubkey_i)…`. We used `[0,0,1,2]`, committed
its blake160 in the 20-byte args, and separately signed with each key. Both
passed actual CKB-VM0 execution; an unrelated signer failed. Our experimental
policy is therefore an **EXISTING SOLUTION** with an inferior representation,
not an invention. This says nothing about equivalence to JoyID's complete
passkey/social-recovery UX.

**CoTA capacity.** `cota-sdk/src/service/registry/index.ts` has
`COTA_CELL_CAPACITY = BigInt(150) * BigInt(100000000)`. It builds one output per
`cotaLocks` entry, a 20-byte type argument derived from its lock hash, and
33-byte data `0x02 || SMT_root`, while continuing a shared registry cell.
This verifies a **SDK allocation constant and construction**, not an independently
measured consensus minimum, a JoyID account-upgrade transaction, or today's UI
price. The README explicitly identifies the syncer that parses witness SMT data
from history and the registry/CoTA aggregators that generate proofs. Operator
failure may therefore be mitigated by rebuilding from history, subject to
availability and exact contract behavior; it is not justified to call every
aggregator a custodian or to assume reconstruction is always instant.

**JoyID evidence limit.** `joyid-sdk/packages/ckb/src/index.ts` exposes main/subkey
witness modes and `getSubkeyUnlock`, which calls `generateSubkeyUnlockSmt`.
`types.ts` describes social-recovery signer sets and recovery modes. The initially
guessed contract repositories were unavailable; the SDK was located using
`@joyid/ckb` package metadata. We did not acquire/execute its full contract or
reproduce the motivating “150 CKB upgrade.” That figure remains unverified as
that specific product operation. The CoTA 150 constant is narrower evidence.
QuotaCell's reported 600 CKB policy cell has no independent source/measurement
here; it is not used as a claimed verified baseline.

**Open Transactions evidence limit.** The pinned Cryptape co-build repository
calls itself a proof of concept. Its `transaction-cobuild-otx-lock-demo` says it
is based on secp256k1/blake160 with co-build/message support and ckb-auth. We do
not label this demo a deployed mainnet standard or collapse all OTX variants
into one security model. No automatic capacity exemption follows from signing
only selected inputs/outputs.

**Fiber.** The inspected light paper, force-close tests, external-funding and
backup guidance show that funding, settlement and monitoring are part of the
system. A payment can avoid a new L1 cell each time by reusing channel state.
This is a strong scaling baseline, but comparing an arbitrary channel limit to
a 150 CKB account cell without measuring both funding and exit paths would be
misleading. We did not run Fiber, measure reserves or claim a numeric advantage
over its implementation.

**Godwoken.** `docs/deposit_and_withdrawal.md` describes deposit lock,
custodian lock, cancellation timeout and finalized withdrawal structures. It
states 298 CKB for CKB deposit and 379 CKB for CKB+sUDT in that documented
version. These are source-reported historical design values, **not** WO01
measurements or a current network quote. They demonstrate why a constant root
alone is an incomplete capital baseline.

**RGB++.** Its lock README explicitly lists RGB++ asset cells, Bitcoin UTXO seals,
Bitcoin SPV-client cells, and BTC-time-lock migration. Its described args contain
Bitcoin txid+output index. Treating its representation as zero CKB state would
omit the asset and verification infrastructure.

## Storage proposals and survey limit

The checked RFC tree includes economic capacity rights/lending (0002/0015), ACP
(0026), Omnilock (0042), and VM/hardfork evolution through 0053. CoTA supplies
an existing commitment aggregation architecture; co-build changes transaction
composition, not occupied-byte accounting. None of the inspected implemented
rules exempts users from the occupied-capacity formula. This is a bounded primary
source survey, not proof that no newer forum proposal, unpublished design or
uninspected experimental branch exists. We make **no novelty claim** that would
depend on such an exhaustive negative. Next-stage work should obtain the actual
JoyID and QuotaCell deployment artifacts and survey new proposals before
claiming superiority to their complete products.
