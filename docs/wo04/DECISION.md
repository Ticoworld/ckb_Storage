# WO04 decision

**RESEARCH OPEN**

The specific unanswered question is: **Across independently operated CKB full nodes, what is the compacted live-state and retained-history footprint per occupied-capacity byte at today's real live-cell distribution, and what conservative byte budget fits hardware that independent operators can actually run?** One local CKB store fixture does not answer this network-wide bound.

## Evidence obtained

The pinned CKB RocksDB schema stores each no-data 61-byte standard-shaped cell as a 185-byte `CellEntry` value and three 36-byte keys. In a 100,000-cell fixture using CKB's configured database options, compaction left about 155–160 file bytes per cell in the three live-cell columns. Five replacement rounds kept the final live set fixed and increased pre-compaction database size to roughly 51 MB; compaction brought it back near the live-set footprint. The harness passed repeated runs, uses CKB's store/database code and defaults, and leaves historical blocks, transaction validation, network sync, indexer data, memory/RSS, and production hardware unmeasured.

This is enough to reject a claim that 61 occupied bytes equal the complete physical node record. It is not evidence for a safe conversion ratio: no current live-cell distribution, complete database profile, operating cost, full-node hardware distribution, or legacy exercise total was available. The test also shows that reducing collateral can increase the number of live outputs funded by the same 61 CKB budget by up to 61 times, without reducing bytes per output.

## Why the model stays open

The public Explorer statistics endpoint returned HTTP 403 in this environment. The source/documentation checkout does not publish current full-node disk/RAM requirements or a historical live-state series. No mainnet sync was attempted. The mainnet genesis and issuance schedule are verified from the pinned source, but current total CKB supply, occupancy share, live cells, and miner security income remain unknown. The WO03 45-billion-CKB placeholder was removed from the new model rather than presented as fact.

## Next falsifiable measurement

Obtain verifiable, aggregate snapshots from at least two independent operators or a published benchmark containing the same pinned CKB revision: current live-cell count and occupied bytes, compacted RocksDB sizes by core/indexer/freezer role, historical-data retention mode, hardware configuration, and sync/replay duration. Compare those observations to the controlled fixture and calculate a conservative node byte budget without assuming one provider's disk is representative. Then re-run capacity ratios against the measured legacy supply and existing secondary-issuance/DAO rules. Until those observations are available, a cheaper protocol ratio has no demonstrated safety or economic advantage.

No consensus fork or deployment is proposed. See [measurements](MEASUREMENTS.md), [issuance and rights](ISSUANCE_AND_RIGHTS.md), and [alternative economics](ALTERNATIVE_ECONOMICS.md).
