# WO04 bounded CKB storage measurements

## Decision boundary and safety

This experiment is based on WO03 `d9e7bf15db94cb759d9652f91b2233d2be8462be`, on branch `wo04-storage-limits`. It does not sync or download mainnet data. The CKB Explorer statistics request returned HTTP 403 through the environment's outbound tunnel; the exact attempt is retained in [`live-stat-access.txt`](../../evidence/wo04/live-stat-access.txt). The work therefore measures CKB's actual database schema with controlled synthetic rows and leaves network-wide safe capacity open.

The CKB source checkout is pinned to `a4aa1286a45bc1d42c30f6698dc51521c3749b40`, the same node revision used for WO01–WO03. The harness inserts the actual packed `CellEntry` representation into the actual `COLUMN_CELL`, `COLUMN_CELL_DATA`, and `COLUMN_CELL_DATA_HASH` RocksDB column families through CKB's RocksDB wrapper and options parser. It writes no blocks, transactions, scripts, block bodies, headers, or historical archives. The serialized output is a standard-shaped 61-byte no-data cell with a 20-byte lock argument. Cell metadata and block identifiers vary deterministically; no live-chain validity is asserted.

The primary run used the node's `default.db-options` and configured 256 MiB block cache. A comparison run used `cache_size=0`; CKB's config warns that this creates per-column-family internal caches unless `no_block_cache=true`, so it is a software-setting comparison, not a low-memory node profile. The fixture ran on the local EPYC 9V74 host (5 visible CPUs, 33 GiB host memory). Its test databases were placed on the workspace overlay filesystem, not a production NVMe. Earlier repeat runs under `/tmp` (RAM-backed here) are retained as file-size reproducibility checks; their timing figures are not disk timing results. The run script checks available space, and at least 4 GiB remained free.

## Direct observations

One no-data cell creates three keyed records in CKB's live-cell store:

- packed `CellEntry` value: **185 bytes**;
- outpoint key: **36 bytes** in each of the cell, data and data-hash columns;
- raw key plus value bytes: **293 bytes per live cell**, before RocksDB table/index metadata and compression.

The RocksDB database has 19 configured column families. An empty fixture database occupied about 354 kB. With 100,000 live cells, explicit compaction produced an incremental **154.8–159.7 file bytes per cell** across the 0-cache and 256 MiB cache runs and their churn cases. This is the file footprint of the three cell columns in this fixture, not a complete node footprint. It is about 2.5–2.6 physical database bytes per 61 occupied cell bytes in this specific dataset. Because it omits every block and history column, it is a lower-scope measurement rather than a complete storage-cost estimate.

Straight-line extrapolation from the 100,000-cell sample gives:

| Live outputs | Logical output bytes | Measured-scope compacted DB extrapolation |
|---:|---:|---:|
| 1 million | 61 MB | 155–160 MB |
| 100 million | 6.1 GB | 15.5–16.0 GB |
| 1 billion | 61 GB | 155–160 GB |

The extrapolation is arithmetic, not a safe-node-capacity estimate. RocksDB compression, compaction, block distribution, actual output data, indexes, the rest of the CKB database, and node hardware can move the result. The machine-readable scenario model explicitly leaves safe admission count and USD storage cost unknown.

## Churn, deletion, CPU and indexing

For the default-cache 100,000-cell cases, insert/delete churn used 0, 1, or 5 full replacement rounds while keeping the final live set at 100,000 cells until the final delete. The pre-compaction database reached roughly **16.1 MB** without churn and **50.6–51.1 MB** after replacement churn; after explicit compaction, the live set returned to roughly **15.8–16.3 MB**. After consuming all live-cell rows and compacting the three columns, **0.65–1.39 MB** remained in those database files. These residual bytes are RocksDB metadata/history of writes in this isolated fixture; the experiment stores no CKB block history and does not claim an archival size.

In this container, one 100,000-cell initial write took about **5.0 seconds** in the default-cache run. A five-round replacement plus final deletion/compaction took about **58.7 seconds**. The timer measures direct database writes and explicit compaction; it does not include CKB transaction verification, block construction, networking, sync, or script cycles. Memory/RSS and sync duration were not measured. The database's configured cache is 256 MiB, but that configuration value is not a measured process-memory requirement.

The shipped `ckb.toml` makes built-in indexer v2 configuration optional and gives it separate database configuration. No indexer was enabled here. Indexing disk and query costs therefore remain unmeasured and must be added separately if independent operators are expected to run it. Historical block and archive costs are also absent from the synthetic live-cell table.

## Economics and the 61-byte rule

The base consensus test remains the WO01 measurement: a fresh standard output occupies 61 bytes and currently requires 61 CKB capacity. The principal 1 CKB is included in that capacity, so the additional capital is 60 CKB, recoverable while the output remains spendable. At `$0.01 / $1 / $10` per CKB that is `$0.60 / $60 / $600` extra collateral. The same 61-byte output at 2, 4, 8, 16, or 61 bytes per CKB would require 31, 16, 8, 4, or 1 CKB total respectively (integer capacity rounding), with the remaining amount above the 1 CKB principal as recoverable collateral. Fees are not included because the database harness does not submit transactions.

At 61 bytes per CKB, a fixed 61 CKB capacity pool can fund 61 such outputs rather than one. Those outputs still create 61 times as many live cell records. The fixture suggests the live-cell database representation is larger than the occupied output bytes, not smaller; it does not establish the full-node, history, or operational cost needed to decide whether any ratio below one CKB per occupied byte is safe. WO03's `$ / byte-year` placeholder is not converted into a cost because this experiment has no power, hardware purchase, disk service life, redundancy, operator, or archival prices. No term rent is quoted from file size alone.

## Exact limitations

No independently verifiable current mainnet live-cell count, aggregate live-state bytes, total outstanding CKB, real node hardware distribution, historical database size series, or full-node bootstrap measurement was available. Official documentation and source code establish the capacity and issuance rules but do not publish those live observations. Public Explorer access was blocked, and no other operators were contacted. The local fixture cannot answer how many cells a decentralized network can safely keep, how archival obligations scale, what the marginal operating cost is, or what conversion ratio preserves legacy redemption and mining security. See [issuance and rights](ISSUANCE_AND_RIGHTS.md) and [decision](DECISION.md).
