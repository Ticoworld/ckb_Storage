# WO03 decision

**RESEARCH OPEN, SPECIFIC GAP**

The specific gap is a defensible, independently audited bound on CKB's long-run safe byte-year budget and marginal resource cost, including validation database overhead, required replication, archival retention, and the persistent anti-replay data required to keep offline monetary claims enforceable. No pinned source or WO01/WO02 run supplies this quantity. WO03 therefore cannot honestly establish that lower upfront capacity is sustainably affordable, nor prove that it is impossible.

## Finding

At current consensus, a fresh standard-shaped 1 CKB output remains 61 CKB: 1 CKB monetary principal plus 60 CKB of recoverable capacity collateral and a separate transaction fee. The user has full capacity capital at risk of opportunity cost, not a 60 CKB storage burn. Existing ACP or shared claims can lower incremental settlement cost but fail the specified fresh direct-L1 output comparison.

A new-format byte-budgeted output could reduce that capital requirement to the 1 CKB principal. That is materially different from a cosmetic lock optimization, but changes consensus accounting and can permit up to 61 times as many minimum outputs for the same capacity-denominated capital. It also requires a legacy-rights reservation rule and replacement miner/security income. The model demonstrates the affordability arithmetic and the state/security sensitivity; it does not demonstrate a feasible equilibrium. Leases fail the offline ownership requirement on expiry unless live anti-replay state remains. Endowments rely on uncertain discount returns. Market credits can become expensive scarce collateral.

## Why the decision is open

The modeled byte-year price ranges are illustrative (`$0.00001` per logical byte-year, 12x replication, 3% real discount), not measurements. Under those assumptions, a 61-byte output's storage service is inexpensive, but the per-user number is not sufficient: very large sybil demand, database/index costs, archive policy, security issuance and a simultaneous exercise of legacy rights determine whether the total obligation is feasible. Selecting assumptions to make the new design pass would manufacture the desired answer. Selecting no cost for the current model would equally conceal the obligation.

## Next decisive experiment

Before implementing a forked validator, produce a reproducible resource budget from diverse CKB node configurations and historical state snapshots: marginal validation/storage/index bytes, replication roles, retention requirements, cost curves, and a conservative maximum safe live-state byte budget. Pair that budget with a verified issuance/DAO snapshot and enumerate the maximum legacy capacity redemption obligation. Then run the same 1m/100m/1bn workloads, sybil churn, price crash, offline-exit retention and miner-security floor through a parameter sweep. If the intersection of affordable user fees, safe bytes, legacy reserves and security income is empty, issue **STOP PROTOCOL REDESIGN**. If nonempty across adverse bounds, implement only the smallest isolated testnet validator rule for a versioned output and rejection/acceptance cases; do not deploy.

## Scope classification

- Current-consensus fresh output affordability: not solved by application scripts.
- New-format capacity conversion: requires consensus changes; exact fork boundary, DAO migration and issuance design remain unresolved.
- Expiry/rent: not acceptable for independent monetary ownership unless anti-replay and exit data stay enforceable through offline periods.
- Legacy rights: no neutral conversion/credit distribution proven.
- Security: no evidence that new fees or storage rents sustain miners/nodes under price divergence.
- Deployment: none. No live fork, devnet proposal, or activation plan is made.

See [first principles](ECONOMIC_FIRST_PRINCIPLES.md), [model results](SIMULATION_RESULTS.md), [legacy rights](LEGACY_RIGHTS.md), and [adversarial findings](ADVERSARIAL_FINDINGS.md). This recommendation is conditional research status, not a breakthrough claim.
