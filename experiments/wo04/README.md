# WO04 fixture

The fixture writes synthetic packed CKB `CellEntry` records to only the live-cell, cell-data, and cell-data-hash RocksDB columns. It does not make blocks or transactions and does not start or sync a node.

Prerequisites: a CKB source checkout at `a4aa1286a45bc1d42c30f6698dc51521c3749b40`, Rust 1.95, the locked Rust dependencies, and at least 4 GiB free on the fixture filesystem. For a fresh CKB build target, keep at least 12 GiB free under `/tmp` to cover the build and retain 4 GiB. The script checks these limits before it temporarily copies `fixture.rs` into `ckb-store/examples`; a trap removes that copy on exit.

```sh
CKB_SOURCE_ROOT=/path/to/ckb-at-a4aa1286a45bc1d42c30f6698dc51521c3749b40 \
CARGO_TARGET_DIR=/path/to/temporary-cargo-target \
WO04_TMP=/path/to/temporary-fixture-db \
./experiments/wo04/run.sh
python3 experiments/wo04/capacity_model.py
```

`WO04_CACHE_SIZE_BYTES` defaults to the node config's 268,435,456 bytes. Set it to `0` to compare the zero-shared-cache configuration. Results are written to `evidence/wo04/store-fixture.csv`; the capacity model consumes that default-cache file and the retained `store-fixture-cache0.csv` run. The scenario rows are arithmetic projections from the 100,000-cell fixture, not safe-capacity or operating-cost forecasts.
