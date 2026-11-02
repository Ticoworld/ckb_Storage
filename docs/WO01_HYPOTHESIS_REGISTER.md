# WO01 — Hypothesis register

Classification applies to the stated hypothesis, not to every possible descendant.
“REJECTED” may reject an advertised guarantee rather than all uses of a mechanism.

| ID | Hypothesis / changed assumption | Ceiling | Boundary / falsification target | Evidence / decision |
|---|---|---|---|---|
| H01 | Put a precommitted recovery branch in the asset lock, no account cell | Eliminate separate recovery registration | Equivalent authority model; compromised owner/recovery can still steal | Executed custom policy and stronger 61 CKB system 1-of-2 baseline. **EXISTING SOLUTION** |
| H02 | Shared root authenticates independent CKB claims | O(1) live metadata for N claims | Sound update, backing, unilateral proof availability, serial exits, no operator authorization | Four independent signers, replay/theft/stale-proof/exit tests executed. Narrow mechanism **EXISTING SOLUTION** (shared UTXO/commitment family) |
| H03 | Empty args provide a secure 41 CKB cell without hidden cost | Save 20+ bytes/cell | Where does independently bound identity live? | Embedded key works, requires 44,677 CKB/code identity. **REJECTED** as general per-user capital breakthrough |
| H04 | Witness supplies all policy, including its own unanchored public key | Eliminate persistent auth commitment | Attacker substitutes its own valid key/policy | No trusted binding distinguishes attacker from owner. **REJECTED** by construction; not deployed as a proposed secure contract |
| H05 | Shorten an existing cryptographic identifier arbitrarily | Direct byte saving | Multi-target/preimage/collision risk; equivalent security | Small hashes trade security; no new compression primitive proved. **REJECTED** as free gain |
| H06 | Refresh shared-tree proofs from public transitions, not operator DB | Remove continuous operator dependency for fixed claims | Does witness disclose changed sibling subtrees? Missed history? | Real VM successor exit and last-claim proof refresh; 240 host index-pair checks. **EXISTING SOLUTION** (authenticated-data-structure update); useful component |
| H07 | Extend public-proof pool to dynamic payments, rotation and revocation | Remove per-user cells while preserving application semantics | Transparent deltas, solvency at admission, fee funding, liveness under adversarial load | Not implemented. **RESEARCH OPEN** |
| H08 | Merkle-sum commitment authenticates total liabilities at creation | Prevent malicious underfunded roots | Type-enforced genesis/updates; bounded arithmetic; root totals; deposits | Plain-root insolvency counterexample executed. Sum-tree/type solution **RESEARCH OPEN**, known building blocks, no novelty claimed |
| H09 | K shared shards avoid one global contention point | O(K) state, concurrent lanes | Rebalance/transfer/exit without trusted coordinator or O(N) hidden cells | Model only. **RESEARCH OPEN**; compare channels/rollups before implementation |
| H10 | Batch independent exits into one transition | Reduce repeated bytes/fees | Users' unilateral route if batch assembler disappears | Not implemented. Existing rollup/shared-UTXO technique. **EXISTING SOLUTION** family, WO01 batching open |
| H11 | Use existing recipient ACP cells for small withdrawals | Avoid new output floor for each payment | Recipient input/auth/deps, existing capital, contention | RFC/source inspection only; prototype forbids extra inputs. **EXISTING SOLUTION**, integration untested |
| H12 | Put policy state into existing typed asset cell | Remove redundant policy cell | Asset compatibility; global vs local counters; occupied balance already funded | No app-specific contract available or executed. **RESEARCH OPEN** integration; generic capability already exists |
| H13 | Stateless daily limit across arbitrary independent cells | Remove counters entirely | Two valid local spends collectively exceed global limit | Verifier lacks spent aggregate unless equivalent authenticated state exists. **REJECTED** for that unconstrained guarantee |
| H14 | Retirement revokes any future deposit to an old static lock | Delete policy immediately | Old owner spends newly created old-lock deposit | Adverse-success test executed. **REJECTED** for static locks |
| H15 | Shared live registry root enforces current recovery policy globally | Amortize current-policy state | Type-ID continuation, required live dep, proof updates, conflicting rotations | Compatible in principle; full protocol unimplemented. **RESEARCH OPEN** |
| H16 | Accumulator replaces Merkle root | Smaller proofs / shared representation | Deletions, witness updates, trusted setup, crypto cost/availability | Root already fixed size; no automatic state reduction. **RESEARCH OPEN**, compare full system |
| H17 | Validity proof lets operator withhold all user data safely | Remove data distribution | Can user construct exit proof after operator disappears? | Correctness != availability. **REJECTED** without separate DA/escape protocol; validium not generally self-custodial under total withholding |
| H18 | Channels eliminate per-payment permanent cells | Reuse funding state over many payments | Liquidity, stale-state disputes, monitoring, mass close | Fiber/channel family. **EXISTING SOLUTION**; quantitative end-to-end comparison outstanding |
| H19 | Sponsorship or leasing makes capacity fundamentally disappear | Zero visible user capital | Count sponsor/lender's assets and reclaim rights | Payer changes; aggregate requirement remains. **REJECTED** as storage breakthrough; useful financing |
| H20 | Native CKB liabilities need less native principal through a commitment | Reduce monetary backing | All users withdraw promised amounts | Ordinary conservation and explicit insolvency case reject undercollateralized guarantee. **REJECTED** without changing claim/liquidity/security |
| H21 | Separate protocol storage credits from native token balance | Decouple dollar affordability from token price | Issuance, scarcity, rent, security budget, migration and existing rights | Consensus/economic change. **RESEARCH OPEN**; no executable fork/equilibrium evidence |
| H22 | Term leases/expiry with recovery guarantees | Price bytes × time; recycle dormant state | Who retains expired claims and pays exit/renewal? | Consensus change unless application voluntarily expires. **RESEARCH OPEN** |
| H23 | Reprice occupied-byte conversion | Lower token capital per byte | State growth, miner income, legacy rights and exchange-rate volatility | Changes consensus accounting; no proven long-run affordability optimum. **RESEARCH OPEN** |
| H24 | Move ownership to another chain's UTXO seals | Reuse foreign authorization | CKB state, SPV costs, foreign fees/reorgs | RGB++ existing representation. **EXISTING SOLUTION**, not state elimination |

## Why these three experiments

H01 tests whether recovery's apparent fixed cost is a protocol rule; it also
provides a cheap falsifiable comparison against an existing lock. H02 has the
highest scaling ceiling because it changes state-per-user, not only bytes.
H03 tests the strongest apparent lower bound counterexample and forces identity
and code-deployment costs onto the same balance sheet. They are different
representations, not three variants of a smaller argument encoding.

After H02's stale-proof rejection, H06 was implemented as a stronger variant
instead of abandoning shared state. After initial reserve overcounting, a
zero-extra-reserve four-claim sequence was executed. After plain-root insolvency
was demonstrated, H08 became the next admission-soundness experiment. No test
failure was relabeled a breakthrough.

## Highest-ceiling next experiments

1. Add a **type-enforced Merkle-sum admission/transition invariant** to the shared
   pool, publish every required delta, and make a user starting with only a local
   proof process an adversarial sequence after the original operator is erased.
2. Extend that protocol to independently authorized payments and key rotation;
   test old credentials, missed histories, late deposits and multi-shard exits.
3. Evaluate storage credits/leases only with a specified fork, migration and
   miner/node cost model; a lower dollar price in a spreadsheet is not evidence
   of sustainable consensus economics.

Only the first two are natural current-consensus follow-ups. They require a
comparison with reusable shared-UTXO/CoTA/channel designs before any novelty claim.
