# Capacity alternatives against measured database bytes

## Same 61-byte output, different capacity rates

The scenario table uses the same fresh 61-byte standard-shaped output at CKB prices `$0.01`, `$1`, and `$10`. It separates the 1 CKB monetary principal from additional recoverable capacity. No transaction fee, annual rent, hardware cost, or future return is invented.

| Bytes per CKB capacity | Minimum total output capacity | Extra capacity beyond 1 CKB principal | Extra capital at $1 / $10 per CKB | Outputs funded by 61 CKB vs current |
|---:|---:|---:|---:|---:|
| 1 (current) | 61 CKB | 60 CKB | $60 / $600 | 1x |
| 2 | 31 CKB | 30 CKB | $30 / $300 | 1.97x |
| 4 | 16 CKB | 15 CKB | $15 / $150 | 3.81x |
| 8 | 8 CKB | 7 CKB | $7 / $70 | 7.63x |
| 16 | 4 CKB | 3 CKB | $3 / $30 | 15.25x |
| 61 | 1 CKB | 0 CKB | $0 / $0 | 61x |

The observed CKB cell-column database requires roughly 155–160 bytes per 61-byte output after compaction in the bounded fixture, before block bodies, indexers or archives. At current 1:1 capacity, that is about 2.5–2.6 measured DB bytes per capacity CKB for this synthetic representation. At 61:1, it would be about 155–160 such DB bytes per capacity CKB, while the same fixed 61 CKB amount could admit 61 times as many outputs. That is a demand and security change, not a physical storage saving.

The model linearly projects measured-scope storage for 1m/100m/1bn outputs. It does not claim the current network has those cells or that the current hardware supports those sizes. The full table is machine-readable in [`capacity-scenarios.csv`](../../evidence/wo04/capacity-scenarios.csv).

## Term pricing and storage credits

The measured DB byte count cannot establish a USD byte-year price. It lacks drive purchase/replace costs, failure redundancy, IOPS, energy, bandwidth, operator labor, hardware distribution, and archive service. A term lease could reduce initial capital only if periodic payment funds those costs and still leaves offline claims enforceable. No rent is proposed until those inputs and renewal/liveness rules are measured.

A storage credit is not free capacity. A global safe limit would make credits scarce; if old holders retain a priority option, that option constrains new credit issuance. WO04 found neither the safe network-wide limit nor current total legacy supply. Any market value or collateral cost therefore remains unknown rather than set to zero.

## Other arrangements

Application-level channels and compact state may reduce replicated on-chain state, but they do not satisfy a fresh independent L1 output without a separate settlement/availability model. A hard byte cap with an auction could bound node state without a price oracle, but legacy holders who exercise capacity can crowd out new users. A capped byte budget with protocol-priced admission is the only distinct direction left that can be stated without assuming indefinite free storage; it still changes access rights and miner economics and is not supported by the missing safe-capacity and demand observations.

No alternative in this section is promoted as cheaper in total social cost. The test establishes a cell-record byte floor for one database subsystem; it does not calibrate rent, market credit scarcity, independent node sustainability, or security income.
