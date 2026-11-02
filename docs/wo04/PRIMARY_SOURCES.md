# WO04 source references

All CKB implementation facts are pinned to `nervosnetwork/ckb` commit `a4aa1286a45bc1d42c30f6698dc51521c3749b40`, identical to the source revision used for WO01–WO03. Per-file SHA-256 hashes and sizes are recorded in [`ckb-source-manifest.json`](../../evidence/wo04/ckb-source-manifest.json).

| Question | Pinned source |
|---|---|
| Output occupied capacity | `util/gen-types/src/extension/capacity.rs`; CKB transaction `CapacityVerifier` in `verification/src/transaction_verifier.rs`; WO01's real validator result establishes 61 CKB for the standard empty-data output |
| Live-cell keys and stored values | `store/src/transaction.rs` `insert_cells` / `delete_cells`; `util/gen-types/src/extension/shortcut.rs` `OutPoint::to_cell_key`; generated `CellEntry` schema in `util/gen-types/src/generated/extensions.rs` |
| Database columns | `db-schema/src/lib.rs`: `COLUMN_CELL` 10, `COLUMN_CELL_DATA` 12, `COLUMN_CELL_DATA_HASH` 14, 19 total columns |
| RocksDB settings | `resource/default.db-options`; `resource/ckb.toml` default cache size 268,435,456 bytes and `options_file=default.db-options` |
| Optional indexer | commented `[indexer_v2]` and separate rich-indexer database configuration in `resource/ckb.toml` |
| Genesis allocations | `resource/specs/mainnet.toml`; extracted values in [`issuance-data.json`](../../evidence/wo04/issuance-data.json) |
| Issuance and halving schedule | `spec/src/consensus.rs`, constants for initial primary reward, secondary reward, epoch length, and four-year halving interval |
| DAO counted capacity | `util/dao/src/lib.rs`, `calculate_maximum_withdraw`; occupied capacity is subtracted before the exchange-rate multiplication |
| Economic rationale | RFC 0015, Final, pinned RFC commit `0c945451c24194f4d34d81195c0159df0a67581a`, indexed in WO01's [`source index`](../../docs/WO01_SOURCES.md) |

The reachable legacy CKB documentation checkout is pinned to `nervosnetwork/ckb-docs` commit `4d5abdfb038c896a5438e7ccfe06e6282daa41dd` (2020-08-25). It describes the token/storage relationship but is not used as a current node-hardware or live-state report. Current hardware requirements and live-chain observations were not published in the source material collected for this task.
