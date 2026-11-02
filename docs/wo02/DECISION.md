# WO02 decision — PIVOT TO PROTOCOL ECONOMICS

## Decision and its precise scope

**PIVOT TO PROTOCOL ECONOMICS. Stop expanding this pool.**

The minimum experiment closes WO01's admission gap and permits conditional
self-custodial shared claims. It does **not** demonstrate a credible capital or
control advantage over the strongest reusable equivalent shared-state approach,
and real ACP already supplies its successful marginal small-credit outcome.
The fixed current-consensus standard-output rule still prevents a gross 1 CKB
claim from independently funding a fresh standard L1 cell at $1 or $10/CKB.

This is **not** a proof that all current-consensus shared ownership is impossible,
or that Fiber/CoTA are inferior. If continued shared or channel ownership is
acceptable, existing-consensus solutions remain useful. The pivot targets the
remaining **fresh direct-L1 affordability requirement**, and asks whether pricing
rules can change without breaking existing rights or sustainable operation.
It does not authorize or recommend deployment of a consensus fork.

## What was proved by executable evidence

- Type-enforced full-manifest admission rejects principal underfunding, false
  roots/totals, reserve shortfall, overflow and copied instance IDs.
- Accepted withdrawals preserve exact remaining liabilities and common reserve;
  24 complete four-user orders pass, with all other claims funded at each step.
- Proof/signature/root/payout attacks and replay/conflicts are rejected by actual
  CKB validators/resolver, rather than only by a spreadsheet or custom model.
- A separate client reconstructs its own proof from retained local state and
  three public packed transactions, with no operator tree/API. The resulting
  locally built signed exit passes real CKB validation. Missing/forged/wrong-tip
  archives are rejected. Initial/current anchors, code and history availability
  remain explicit assumptions; no live light-client/DA guarantee was proved.
- A 1 CKB gross claim credits **0.9999 CKB** into a real existing ACP cell; the
  recipient then independently spends it. A fresh standard output rejects it.

There are **142 core cases: 110 accepted, 32 expected rejections**, plus four
separate archive-client attack checks, 144 proof refreshes and a 256-real-key
admission. The mechanism is capped at 256 initial slots and implements immutable
claims/withdrawals only. No payments, wallet UI, elaborate recovery or shard
coordination was added.

## Why this does not pass ADVANCE SHARED STATE

The type repairs a known missing invariant with a known authenticated sum-tree
technique. It does not establish a new scaling relationship over an equivalent
reusable shared-UTXO design. Compared with the direct ACP baseline under the same
recipient, gross budget and fee, the credited amount is identical, while the
pool fixture is **853 bytes / 1,973,987 cycles**, versus **590 bytes / 1,737,001
cycles** for direct ACP. It adds a 147 CKB root/reserve policy and **65,322 CKB
of immutable shared code**, not a lower marginal ACP capital requirement.

That new code capital is irrecoverable in the modeled immutable deployment.
A reclaimable owner-lock deployment costs 40 CKB more and exposes code-removal
liveness risk; it cannot silently inherit the same operator-failure guarantee.
The donated common reserve goes to the final claimant; its capital and last-exit
incentive are counted rather than attributed to a free operator subsidy.

Shared holding can reduce replicated metadata. At 256 equal one-CKB claims,
403 CKB of live pool capacity is about 1.57421875 CKB per shared claim before new
code allocation. But 256 fresh standard recipient outputs still require at
least **15,616 CKB**, while existing ACP recipients supplied their 61 CKB each
beforehand. This representation/control distinction cannot be erased by moving
proofs into witnesses.

Multisig already supplies WO01's tested recovery model in 61 CKB. CoTA's token/
extension aggregation and Fiber's actual payments/liquidity/dispute capabilities
were not reproduced here; removing their features is not evidence of superiority.
A complete equivalent end-to-end benchmark would be required before claiming a
win over either. We have enough to reject an advance claim, not enough to prove
all future shared architectures inferior.

## Economic boundary at high prices

A fresh 61 CKB standard output needs **60.0001 CKB extra native capital** beyond
a gross 1 CKB claim at the measured fee: about $60.0001 at $1/CKB or $600.001 at
$10/CKB. The 61 CKB output requirement is current consensus behavior.
An existing ACP cell avoids new capacity at exit because it already holds at
least $61/$610 of recipient capacity under those prices. Continuing a shared
claim avoids that fresh output but retains shared-state/proof/availability/
serialized-exit conditions, and this prototype has no internal payment feature.

Making the recipient's pre-existing capital, reserve, code or archive provider
invisible would not solve this economic objective. Lowering a few bytes or
optimizing the conservative reserve would not change the standard output floor.
Therefore the pool work stops at this decisive experiment.

## Why pivot rather than STOP

A credible, separately falsifiable hypothesis remains: decoupling monetary
balances from priced storage rights, bounded term leases, or a changed storage
conversion might lower upfront fresh-L1 capital while preserving legacy rights,
resource bounds and the security budget. Current scripts cannot waive that global
occupied-capacity check. These alternatives require explicit protocol rules and
incentive/migration analysis; their success is not assumed.

[PROTOCOL_ECONOMICS.md](PROTOCOL_ECONOMICS.md) specifies the rule changes,
rights/migration issues, miner/node incentives, availability and expiry risks,
and the next parameterized feasibility test. No consensus fork was implemented.
If no feasible parameter range survives that test without subsidies hidden from
the balance sheet or rights violations, the next decision should be **STOP**.

## Review and reproduce

- [Competitive threshold fixed before implementation](COMPETITIVE_THRESHOLD.md)
- [Protocol and invariant](PROTOCOL.md)
- [Results, scope and retained failures](RESULTS.md)
- [1 CKB costs and equivalent ACP comparison](ECONOMICS.md)
- [Reproduction instructions](../../experiments/wo02/README.md)
- [Raw output](../../evidence/wo02/validator-run.txt)
- [Machine-readable cases](../../evidence/wo02/validator-results.json)

WO01's verified base, proven results, rejected hypotheses and executable evidence
remain intact. The recommendation applies to this tested branch and the named
objective, not an inflated architectural-breakthrough claim.
