# WO01 — Breakthrough decision

## Decision

**No architectural breakthrough is supported by this evidence.**

The investigation disproves some overly broad assumptions, but the successful
mechanisms have known reusable equivalents or unresolved requirements. A secure
small cell alone does not establish a lower-cost system.

| Serious candidate | Classification | Quantified finding | What prevents a stronger claim |
|---|---|---|---|
| Recovery inside existing asset authorization | **EXISTING SOLUTION** | Bundled 1-of-2 multisig gives owner OR recovery authority in a 61 CKB cell; custom experiment needs 73 CKB plus 52,621 CKB shared code | No advantage over reusable baseline; both have immediate spending-key compromise risk and static-lock late deposits |
| Fixed independently signed claims in one live Merkle root | **EXISTING SOLUTION** | 73 bytes live root metadata for multiple users; real independent exits; no-extra-reserve 400 CKB backs four 100 CKB claims | Shared-UTXO/commitment architecture already exists; immutable claims only, contention, proof-history assumptions and unvalidated admission |
| Dynamic shared state with solvency, recovery, revocation, payments and unilateral exits | **RESEARCH OPEN** | One depth-20 exit is executable at 1,136 packed bytes and 2,278,569 cycles; public proof refresh works for the simpler fixed-claims transition | No dynamic implementation, sum/admission invariant, end-to-end DA/failure proof, mass-exit workload or strongest-product superiority demonstrated |
| Per-owner embedded-key code with empty args | **REJECTED** as general independent-user breakthrough | Secure 41 CKB cell, but approximately 44,677 CKB unique code per identity in this implementation | Moves identity capital into deployment; repeated same-key cells are a different workload |
| Separate protocol storage rights / term leases | **RESEARCH OPEN** | Price model identifies dollar affordability pressure and accounting choices | No specified implemented fork, incentive equilibrium, migration or rights preservation proof |

No entry qualifies as **BREAKTHROUGH CANDIDATE**. An open extension is not promoted
just because its root has constant size.

## Strongest positive finding

A separate persistent registration/recovery cell per user is not a consensus
necessity. More broadly, one cell can authenticate several independently signing
users, and the withdrawal-only experiment lets a user update its own Merkle
proof from public exit witnesses without the operator's database. Real CKB
verification accepts the refreshed exit. No operator key is in its authorization
path. This challenges a per-user live-cell assumption with actual script logic,
not only a storage formula.

That finding is narrow. State commitment and local witness update are established
ideas; this experiment does not claim to invent them. For ordinary 100 CKB
balances, standard cells already hold the principal underwriting their bytes.
The pool removes replicated occupied metadata but does not reduce `100N` native
monetary backing; its new code adds 57,189 CKB. For small 1 CKB claims, a million-user
formula falls from 61,000,000 CKB of standard cells to 1,057,261 CKB including new
pool code, **but the prototype cannot fund equivalent fresh independent L1 exits**.
That is a changed settlement/security tradeoff, not a proven 98% breakthrough.

## Strongest falsification

A plain lock-only pool can be created with a valid root promising 400 CKB but
only 173 CKB of funding. Real creation validation accepts it, the first 100 CKB
withdrawal accepts, and the next cannot be funded. Membership authentication
never established total solvency. The present pool is safe only under explicitly
verified initial funding and proof-distribution assumptions; it is not a general
untrusted deposit protocol.

Other decisive counterexamples remain visible: competing valid exits cannot
both consume the same root; stale proofs reject unless refreshed; no witness
means no unlock; a recovered cell does not revoke future deposits to an old lock;
empty args do not eliminate supporting identity data; a native balance cannot
be replaced by an undercollateralized hash without losing a guarantee.

## Improvement categories achieved

1. **Eliminates unnecessary persistent state:** recovery can be precommitted in
   an asset lock; this is already available through the tested system multisig.
2. **Reduces necessary state bytes:** empty args work with embedded identity,
   but total capital is worse for independently keyed users here.
3. **Amortizes state across independently secured users:** fixed solvent claims
   share 73 live bytes conditionally; code, claims backing, proof availability,
   serial updates and fresh-exit capital remain counted.
4. **Improves safe capacity recovery:** all four funded claims close and the
   root retires; this validates a path but does not demonstrate a novel better
   protocol. Universal old-address retirement is disproved for static locks.
5. **Changes normal-operation capital:** small-claim pooled representations can
   change the formula, but tested equivalent exit/recovery semantics are missing.
   No reduced monetary backing for native balances was achieved.
6. **Better protocol-level economics:** no implemented or validated design yet.
   Token/byte coupling is an economic choice, not proved optimal or disproved
   sustainable by three hypothetical dollar prices.

## Next decisive experiment

Implement a **type-enforced Merkle-sum shared state** with an authenticated total,
creation/admission rules and transparent transition deltas. Initially keep fixed
claims so admission and availability are separately falsifiable; then add signed
internal payments and owner/recovery rotation. Set the failure model before
claiming self-custody:

- users retain only their own initial authenticated claim/proof and credentials;
- initial operator/database is deleted after admission;
- confirmed chain transactions/witnesses and code can be retrieved from independent
  archives; explicitly test what happens if that availability assumption fails;
- hostile users race, forge, replay, withdraw and delay publication;
- users must construct exits after missed updates, without private operator help;
- full liabilities, reserves, recipient infrastructure, code and exit working
  capital are counted at every state.

First acceptance criterion: underfunded or overflowed roots cannot be admitted
as solvent, and valid remaining claims stay funded after **any** accepted exit
order. Second: user-only reconstruction remains executable after the original
state provider disappears. Third: demonstrate fresh-output and existing-ACP
exit variants with honest capital accounting and a realistic adversarial queue.
A root that passes membership checks but fails one of these is not enough.

Compare the resulting protocol against CoTA shared facts, reusable shared-UTXO
constructions and Fiber/channel settlement with the **same** operator-loss,
credential-compromise, composability, liquidity and exit assumptions. Acquire
actual JoyID/QuotaCell deployment artifacts before claiming product superiority.
Mass-exit testing should populate real accounts and exercise real transaction
ordering, rather than multiplying one depth-20 measurement.

This is a concrete research continuation, not completed evidence. WO01's tested
results, limitations, primary sources and failed assumptions are preserved on the
dedicated research branch so the next experiment can begin from a reviewable base.
