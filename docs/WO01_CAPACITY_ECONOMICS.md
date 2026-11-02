# WO01 — Capacity economics

## Units, assumptions and evidence

One occupied byte requires one CKB (100,000,000 shannons) under the inspected
rules. This is locked capital/opportunity cost, not a one-time destruction of
one CKB. Transaction fees are separate. Releasing state can make capital usable
again, but fresh outputs and outstanding claims still need funding.

`experiments/model.py` reads the real validator's occupied-capacity measurements
and compiled ELF lengths, producing [120 scenarios](../evidence/capacity-model.csv),
[capacity tables](../evidence/CAPACITY_TABLES.md), and
[fee illustrations](../evidence/fee-scenarios.json). Every population total is a
**formula extrapolation**, not a million-account execution. Proof-depth timings
execute one claimant per size using synthetic sibling-subtree commitments.

We compare equal native balances `b` and `N` independent users:

```
standard single key or existing 1-of-2: N * max(61, b)
custom two-key policy:                N * max(73, b) + 52,621 new shared code
embedded independent key:             N * (max(41, b) + ~44,677 unique code)
fixed homogeneous shared pool:        N*b + max(0, 73-b) + 57,189 shared code
```

The pool reserve expression covers the live-root floor when only one equal-sized
claim remains before closure. It is conditional on this prototype's single-exit
rules and excludes fees beyond the hardcoded amount paid by each claimant. For
`b=100`, no extra reserve is needed: all four exits from a **400 CKB** cell pass.
The earlier 473 CKB fixture is deliberately retained to show that its donated
73 CKB reserve goes to the final claimant at closure. It is not necessary capital
for that 100 CKB case. For `b=1`, 72 extra CKB keep the remaining root valid, but
nonfinal 1 CKB fresh-output payouts still fail; a reserve is not a cure for that.

A balance and its cell's occupied bytes are **not additive deposits**. A 100 CKB
standard cell already funds its 61 occupied bytes. Charging `100+61` per user
would invent a false gain for aggregation. Likewise, pooling native CKB liabilities
cannot reduce their monetary backing without changing solvency, liquidity or
claim semantics.

## Full supporting infrastructure

| Component | Measured data/ELF bytes | Occupied CKB with 20-byte deployment lock |
|---|---:|---:|
| Custom two-key policy | 52,560 | 52,621 |
| Shared pool | 57,128 | 57,189 |
| Embedded owner | 44,616 | 44,677 |
| Bundled standard sighash | 52,048 | 52,109 |
| Bundled standard multisig | 52,424 | 52,485 |
| Shared secp256k1 data | 1,048,576 | 1,048,637 |

These are data lengths and official occupied-byte arithmetic, **not actual new
on-chain deployments**. The mainnet genesis configuration has already deployed
system code and secp data, with configured capacity that can exceed occupied
minimum. We do not charge new users for redeploying that infrastructure. The CSV
also reports full relevant occupied infrastructure: sighash+data = **1,100,746**
CKB; multisig+data = **1,101,122** CKB. If both are used, the data is shared and
must not be charged twice. Actual configured genesis holdings are separately
inspectable in the pinned mainnet spec; occupied minima are not those holdings.

The pool's standard recipient lock eventually needs that existing system
infrastructure to spend. Experimental code counts as new capital even if someone
else deploys it. The embedded-key extrapolation assumes equal-size binaries for
different independent keys; only one was compiled. Shared embedded authority
is not equivalent to independently controlled users.

## Scaling results

Totals below count newly required code but exclude the already deployed system
infrastructure. Transaction fees and working capital for congestion are excluded.

| N | Existing 1-of-2, b=1 | Custom policy, b=1 | Pool, b=1 (fresh exits fail for N>1) | Pool per user incl. new code |
|---:|---:|---:|---:|---:|
| 1 | 61 | 52,694 | 57,262 | 57,262 |
| 10 | 610 | 53,351 | 57,271 | 5,727.1 |
| 1,000 | 61,000 | 125,621 | 58,261 | 58.261 |
| 1,000,000 | 61,000,000 | 73,052,621 | 1,057,261 | 1.057261 |

The apparent 98.27% million-user reduction is **not an equivalent-security
breakthrough**: these small claims cannot each exit to a fresh standard cell
without additional capital, and the prototype lacks admission solvency enforcement,
dynamic payments and recovery. The b=1 baseline owns capacity beyond its intended
small spend balance; its owner may reclaim it by consuming/closing that cell.

For b=100, existing minimum total is `100N`. The pool is `100N + 57,189` including
new code. Occupied user metadata falls from `61N` bytes to **73 bytes**, but
normal-operation monetary capital does not fall. No extra reserve is required.
At N=1,000,000 this is **100,057,189 CKB** versus **100,000,000 CKB** before common
infrastructure/fees. The new code adds about **0.057189 CKB/user** when amortized.

Embedding one unique key in code costs about **44,718 CKB/user** at b=1; at one
million independent users that formula exceeds 44.7 billion CKB and is not a
feasible deployment forecast. One identity reusing code over K cells could
amortize 44,677 CKB against 20 CKB saved per cell (break-even about 2,234 cells,
ignoring fees and size changes). That is a different workload, not one million
independent accounts.

## Hypothetical dollar prices

| Capital requirement | $0.01/CKB | $1/CKB | $10/CKB |
|---|---:|---:|---:|
| Standard owner or 1-of-2 cell, 61 CKB | $0.61 | $61 | $610 |
| Custom policy cell, 73 CKB (excluding code) | $0.73 | $73 | $730 |
| Motivating 150 CKB allocation | $1.50 | $150 | $1,500 |
| Unverified motivating 600 CKB policy cell | $6 | $600 | $6,000 |
| Million b=1 existing cells, 61,000,000 CKB | $610,000 | $61,000,000 | $610,000,000 |
| Million b=1 pool + new code, 1,057,261 CKB, restricted guarantees | $10,572.61 | $1,057,261 | $10,572,610 |

The 150 CKB CoTA SDK allocation is verified as a source constant, not JoyID's
particular account-upgrade transaction. The 600 number remains unverified and
is not included in measured comparative totals. Prices are hypothetical, not
forecasts or market observations. Lending/sponsorship shifts who supplies these
amounts; it does not remove them from the system.

## Exit, concurrency and recovery of capacity

A million fresh standard L1 output cells require at least **61,000,000 CKB**
aggregate output capacity, plus fees, irrespective of a preceding tiny root.
A million 1 CKB claims cannot alone fund them. Recipients with existing ACP
cells or users accepting a shared/channel settlement can avoid a new-cell
increment, but that is a different exit representation and its prior capital
and dependencies must be counted. The one-input pool does not implement it.

The pool makes one state transition per exit and all transitions consume a common
outpoint chain. Conflicting siblings cannot both resolve. Chained descendants
can potentially appear in one block in dependency order; **one exit per block is
not a consensus rule**. Production throughput and fairness also require proof
refresh, transaction assembly, propagation, fee bidding and block inclusion.

At the million-user proof depth we measured 1,136 packed transaction bytes and
about 2.28 million cycles for **one** exit. Multiplying that representative shape
by one million gives about 1.136 GB of packed transactions and 2.28e12 cycles,
not a measured mass exit. Against source defaults of 597,000 block bytes and
3.5e9 cycles, byte division alone suggests about 1,903 block-equivalents before
block overhead, competing use and final-exit size changes. This is an optimistic
resource illustration, **not** an exit deadline, throughput benchmark or guaranteed
inclusion time. No million-state history was replayed.

All 4 solvable test claims close. With the donated reserve fixture, three payouts
are 100 CKB minus 1,000 shannons and the final payout is 173 CKB minus that fee;
with the no-extra-reserve fixture, each receives 100 CKB minus the fee. Closure
eliminates the root's occupied 73 bytes. It does not give those bytes a second
capital refund in addition to the payouts. Recipient cells remain live. Shared
code cannot be retired safely while clients still need it unless equivalent
code availability and redeployment funding are provided. We did not transact
actual code deployment/retirement.

## Fees, DAO and protocol alternatives

Each ordinary signed experiment uses a deliberately fixed **1,000-shannon** fee
(0.00001 CKB); capacity-boundary probes intentionally burn more and are not fee
benchmarks. Some largest packed transactions exceed 1,000 bytes, so the fixed
fee is **not** evidence of admission at a 1,000-shannon/kB pool policy. The fee
JSON gives illustrative byte-based rates and discloses that admission was not
tested. CKB transaction weight also accounts for cycles; real fee decisions
must use exact policy/size methods and observed network conditions.

DAO compensation is not a quoted fiat yield. Its source calculation rewards
counted capacity excluding occupied bytes using accumulated-rate accounting.
Locked asset capacity cannot simultaneously be assumed available for arbitrary
application state and to earn full DAO treatment. An illustrative opportunity
cost `capital × assumed annual rate × time` is a model, not a measured return.

| Protocol alternative | Possible affordability effect | Necessary costs and rights to specify | Status |
|---|---|---|---|
| More bytes per CKB | Lowers token capital per byte | Larger state growth, hardware burden, security budget, effect on existing capacity rights | Requires consensus change; no optimal ratio established |
| Separate scarce storage credits | Separates monetary balances from storage inventory pricing | Credit issuance/trading, funding, issuance authority, rent, Sybil resistance, migration | Requires consensus design; price can still rise with demand |
| Renewable term leases | Charges for bytes × time, recycles abandoned state | Expiry notices, permissionless exits, renewal, archive/DA and lost-key treatment | Voluntary app-level expiry possible; compulsory global expiry requires consensus change |
| Fiat-indexed pricing | Attempts stable dollar onboarding cost | Trusted/oracle manipulation, volatility, repricing shocks, existing claims | Speculative fork; not proved sustainable |
| Public data subsidy / quotas | Low visible onboarding cost | Someone pays node/miner costs; scarce identity or Sybil abuse bound | Financing/policy; no automatic storage breakthrough |

Application-level expiry still requires a valid transaction to consume cells; it
does not automatically garbage-collect state or waive capacity.

None is rejected simply because today's design differs. None is declared better
without a specified transition system, attack model, migration and security-budget
analysis. This work does not supply those missing economic proofs.
