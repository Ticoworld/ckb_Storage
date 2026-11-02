# Security budget and state growth

## Resource accounting

At the 61-byte standard shape, identical workloads of 1 million, 100 million and 1 billion outputs create 61 MB, 6.1 GB and 61 GB of logical live state respectively (decimal units). This is only the cell-output bytes, not database indexes, write amplification, snapshots, network replication, caches, backup copies or historical data. WO03 applies an explicit 12x physical replication equivalent in its annual cost sensitivity, but this is a scenario input, not a measured CKB node multiplier. Validator, full-node and archival responsibilities differ; the model does not disguise archive obligations as full-node obligations.

Under the 61-byte/CKB conversion, the same workload still has those bytes. The per-user byte requirement does not magically grow 61x at fixed user count; rather, the same 1 CKB balance can now obtain 61 bytes instead of the former 1 CKB of capacity. If lower capital enables 61 times more users, state grows 61 times. At 1 billion such users that is 61 GB logical output data, before metadata and indexes. A hard budget prevents unlimited growth but means not every 1 CKB holder can receive a fresh output at once.

The model sets an illustrative direct resource cost of `$0.00001 per logical byte-year` and 12x physical replication equivalent. A 61-byte output costs `$0.00732 per year` under that assumption, or `$0.0732` over ten years at flat cost, and `$0.244` present value forever at a 3% real discount rate. A 50% hardware-cost shock scales those values by 1.5; a 10% annual decline is treated as an immediate 0.9 multiplier and held constant thereafter. These figures are **not estimates**. They expose the formulas and show why measured marginal long-run node cost is a decisive missing input.

## Security income stress

The harness compares hypothetical annual network security budgets of `$1m / $10m / $100m` with the visible fee revenue from the WO02 fixture fee (`0.0001 CKB` per operation). One million operations at `$0.01/$1/$10` per CKB yield only `$1/$100/$1,000` at that fee. At 1 billion operations the same arithmetic is `$1,000/$100,000/$1,000,000`. It is not a complete CKB security model: issuance, other fees, MEV, miner cost and attacker economics are not estimated. The comparison makes explicit that low fees alone do not finance the stress budgets.

RFC 0015 ties a portion of secondary issuance to occupied capacity. If conversion reduces counted occupation per monetary claim, miner income under that component changes unless issuance/accounting changes. Paying nodes and miners from a single rent stream requires an explicit split and cannot count the same dollar twice. A crash in CKB market price scales native-denominated security income in USD immediately while hardware/resource obligations may not fall. Conversely, a rise in storage demand can raise credit prices without raising token issuance unless the protocol directs the value to miners.

## State growth attacks and liveness

- **Sybil creation:** A cheap new format supports one billion accounts or repeated create/retire churn. Capacity can be capped by byte budget, but sybil-resistant priority needs scarce native value, identity, proof of work, or auction bids. Each changes access or introduces costs.
- **Dormancy:** An expired lease releases physical live state only if old claim data is unnecessary. If the user must later prove monetary entitlement, retain enough commitment/nullifier/index data and a data-availability path. If not retained, funds are unspendable; if retained indefinitely, the protocol still carries state.
- **Renewal failure/censorship:** For a lease, user must renew during a bounded window. If censored or offline, either state is deleted (loss of independent direct control) or grace/exit state persists and needs funding. A third-party keeper can improve liveness but adds a dependency.
- **Mass creation and retirement:** Annual state cost for N users is linear in N under the assumed bytes, before churn. If each create/retire cycle writes history, archival cost can grow even when live state returns to baseline. Ethereum EIP-4444's historical pruning is not a deletion rule for live balances.
- **Credit exhaustion:** With a 10% spare-credit stress assumption and a 10x scarcity multiplier over a 1 CKB/byte base, a 61-byte output's credit costs 610 CKB extra. This is intentionally hostile and not asserted as equilibrium. It shows that a tradable credit can erase the apparent affordability advantage when demand approaches the cap.
- **Price crash:** If rent/endowment is denominated in CKB, a 90% price decline makes its fiat resource budget one tenth as large. Indexing to fiat requires an oracle or external asset; an endogenous storage-token market can still reprice unpredictably.
- **Offline holder exercise:** If legacy rights are reserved only for occupied cells, a dormant holder cannot reconstruct the original option. If all holdings are reserved, worst-case bytes may exceed practical node capacity. This is a governance/rights choice, not a hash-function issue.

## Recovery and archive obligations

An output owner needs keys and chain data sufficient to construct a valid spend; current CKB cells are validated from chain state and spend authorization, not by an operator's database. If balances are compressed into expiring or off-chain receipts, enforceability requires durable commitments, replay protection, spend-nullifier state, and data/anchor availability. Those costs were already demonstrated as material assumptions in WO02 and are not re-tested here. WO03's lease rows explicitly fail the ownership objective at expiry if renewal is missed; they are not counted as successful cheaper ownership.
