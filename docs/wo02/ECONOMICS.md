# WO02 — The 1 CKB objective at $1 and $10

## Compare ownership outcomes first

A **gross 1 CKB claim** means the pool owes one CKB before its exit fee. In the
measured implementation, a 10,000-shannon fee leaves **0.9999 CKB** for the
recipient. A separate full-net-one-CKB comparison must fund the fee in addition.
Neither convention lets a root create native monetary capital.

| Outcome for the 1 CKB claim | Incremental exit requirement | Already needed | Operator/control condition |
|---|---|---|---|
| Fresh independently spendable standard L1 cell | At least **60.0001 extra CKB** beyond gross claim to produce a 61 CKB output and pay the fixture fee | Existing standard code/data; admission/pool capital if sourced from pool | Minimal pool has no top-up input integration; actual 1 CKB exit is rejected by capacity. Additional capital would be someone's native assets |
| Existing recipient ACP cell | No extra cell-capacity deposit; net credit 0.9999 CKB | **61 CKB** recipient capacity, ACP code/data, user recipient key for later spend | No recipient signature needed to accept credit. Real ACP validation and later independent owner spend both pass |
| Continue holding the shared claim | No exit transaction/fee now | Native 1 CKB backing plus allocated common reserve/code; authentic local proof/public history | Conditional independently enforceable shared ownership, not fresh L1 output or implemented internal payments |

The new standard-cell shortfall is `61 - (1 - 0.0001) = 60.0001 CKB`. This is an
accounting lower bound for the named standard representation, **not** a tested
working top-up feature or proof that every conceivable ownership representation
requires 61 CKB per user. A sufficiently different settlement representation may
avoid it; that must be compared as a different outcome.

| Item | At $1/CKB | At $10/CKB |
|---|---:|---:|
| Gross claim | $1 | $10 |
| Pool/direct-ACP fixture fee | $0.0001 | $0.001 |
| Net 0.9999 CKB credit | $0.9999 | $9.999 |
| Existing 61 CKB recipient capacity | $61 | $610 |
| Additional fresh-output capital beyond gross claim, 60.0001 CKB | $60.0001 | $600.001 |
| Total fresh-output gross capital, including fee, 61.0001 CKB | $61.0001 | $610.001 |
| Whole pool reserve, 147 CKB (not per user) | $147 | $1,470 |
| New immutable type+lock code, 65,322 CKB (not per user) | $65,322 | $653,220 |

Prices are hypothetical. No exchange prices, future yield or fee-market quote
was used. Full machine-readable scenarios are in
[`one-ckb-economics.csv`](../../evidence/wo02/one-ckb-economics.csv).

## Actual ACP comparison

For an existing 61 CKB recipient:

- Pool: gross liability drops by 1 CKB; recipient gains 0.9999 CKB; fee 0.0001.
- Direct ACP, same gross budget: sender drops by 1 CKB; recipient gains
  0.9999 CKB; fee 0.0001.
- Direct ACP, full net credit: sender drops by 1.0001 CKB; recipient gains
  1 CKB; fee 0.0001. This is separately tested, not confused with equal gross cost.

The direct same-budget transaction is **590 bytes / 1,737,001 cycles**, versus
**853 bytes / 1,973,987 cycles** for the pool exit in the retained run. The pool
adds state/type/code and proof history; it does not reduce marginal ACP credit
capital below one native CKB of gross funding. Its source principal may be
amortized across users while they remain shared; that is an existing shared-UTXO
advantage, not an ACP invention.

The fixture donor already holds a standard funding cell, a pre-existing source
of liquidity. The pool similarly needed a funded admission input. Neither side
is allowed to omit source funding or call the recipient's 61 CKB free. If a user
has no owned ACP cell and credits a service's cell instead, the result is service
control, not the independently controlled L1 exit tested here.

## Pool reserve, principal and code

Protocol root occupancy is **147 CKB**, and this experiment additionally keeps a
fixed **147 CKB common reserve** inside the pool alongside liabilities:

```
pool.capacity = sum(gross remaining claims) + common reserve
```

This is the actual checked invariant, not `sum + occupied + reserve` (which would
double-count the same root funding). The reserve is deliberately conservative,
as explained in PROTOCOL.md. It could be reduced for a restricted minimum claim
or redesigned refund policy, but that does not eliminate fresh recipient output
capacity. At closure, the last claimant receives it; it is a real capital
contribution and last-exit windfall, not an unowned accounting entry.

The executed four-claim admission has balances **1, 2, 3, 4 CKB**: 10 CKB of
principal + 147 CKB reserve = **157 CKB live pool capacity**, plus the admission
fee and shared code. All 24 orders pay every gross claim less its fee; the last
also receives the reserve. No operator reclaim path is hidden behind that policy.

For equal 1 CKB claims, the following are **allocation arithmetic**, not claims
of implemented payments or coordinated multi-pool operations:

| Claims in one pool | Principal | Common reserve | Live capacity | New code once | Live+new code per claim |
|---:|---:|---:|---:|---:|---:|
| 1 | 1 | 147 | 148 | 65,322 | 65,470 |
| 4 | 4 | 147 | 151 | 65,322 | 16,368.25 |
| 256 | 256 | 147 | 403 | 65,322 | 256.73828125 |

All figures are CKB. The **256-claim admission** with actual independent keys and
403 CKB capacity was executed. Only the heterogeneous four-user exit sequences
were exhausted. At that one-pool ceiling, even new-code amortization costs more
than 61 CKB/user before recipient infrastructure. Already deploying/reusing code
across many separate pools could amortize it further, but that is not a feature
unique to WO02 and no multi-pool coordinator or transfer protocol was built.

If code were already deployed, the equal-256 shared live capacity would be
**1.57421875 CKB/claim**, not 61 CKB. That is a credible storage-capital reduction
for a **shared claim** with the stated availability and serialized-exit conditions.
It is not equivalent to 256 new fresh 61 CKB recipient cells; those require
**15,616 CKB** of recipient capacity when all independently exit. Existing ACP
cells avoid creating that capacity at exit because someone supplied it earlier.

## Infrastructure and retirement

Measured code minima under a **20-byte owner lock** are type 64,957 CKB,
always-success lock 405, ACP 66,109,
standard sighash 52,109, secp data 1,048,637. The tested lifecycle's existing
standard+ACP+secp occupied minima total **1,166,855 CKB**, shared once. Standard
code is required for the funding/recipient-spend baseline; ACP is needed for
that exit route. A never-exiting static shared claim need not load ACP for its
own state verification. Already deployed infrastructure and newly required
65,322 CKB **immutable burn-lock** type+lock capital are listed separately, not charged twice per user.
No actual on-chain code deployment, configured genesis holdings or live ACP
network deployment was measured by this experiment.

The modeled new code cells use zero-code-hash unspendable locks, matching the
fixture deployment model: 64,937 CKB type + 385 CKB lock. Their **65,322 CKB is
irrecoverable**, even after all pools close. Underlying hash/preimage assumptions
and live code availability remain required. Reclaimable 20-byte owner locks cost
40 CKB more and let that owner remove the only code cells; that variant has an
additional liveness trust/capital dependency and is not the self-custody baseline.

Closure consumes the root and pays its remaining capital; recipient cells remain
live. Shared code is not counted as recovered while clients may need it, and
there was no actual code-cell retirement transaction. Data-hash references
prevent silent code substitution, but destroying the only code cell can still
harm liveness unless code availability and redeployment capital are arranged.
Code availability/control and archive service remain dependencies, not free
operator promises. Production immutability/reclaim arrangements were not audited.

Fees reported here are exact input/output differences in the fixtures. There is
no network admission/throughput or forced-inclusion deadline claim. Separate
proof refresh and claimant signing still require public history and user action;
conflicting root spends serialize. No per-day spending policy, dynamic internal
payment protocol or cross-shard concurrency was added to make the comparison
look stronger.

## Competitive result

WO02 removes the plain-root admission weakness but supplies **no economic or
control advantage over the same reusable type-enforced authenticated shared-UTXO
construction**. ACP already provides the tested cheap marginal credit. CoTA has
different token/extension semantics; Fiber provides payments/liquidity/dispute
machinery this fixed-claim pool lacks. Their complete equivalent workloads were
not benchmarked, so inferiority or superiority to them is not asserted.

For a mandatory fresh standard L1 ownership outcome at $1/$10, the unchanged
consensus floor dominates. For users accepting continued shared/channel claims,
existing-consensus aggregation remains a valid option; no fork is proved
necessary for that weaker/different outcome. The experiment therefore does not
justify expanding the new pool. See [DECISION.md](DECISION.md).
