# WO04-R1 decision

## RESEARCH OPEN

**We still need independent full-node operator measurements before proceeding to a credible storage-cost or safe-conversion range.** The DAO field is a consensus ground-truth source for occupied-capacity growth and cumulative issuance, and the exact decoding and genesis exception are now established from RFC 0023 and pinned CKB code. However, this environment's restricted proxy denied both attempted public RPC hosts, so this run could not produce independent tip/historical header observations. No current live-cell count or node-size data was obtained.

Even a successful pair of header series would not settle the physical-cost question: `U_i` counts occupied capacity under protocol rules, while the WO04 155–160 bytes/cell result covers only a synthetic fixture and selected RocksDB columns. Before revisiting global capacity pricing, collect (a) matched `U_i`/`C_i` header observations over dated checkpoints from two verifiably independent providers and (b) operator or controlled-node measurements reporting live-cell count, occupied bytes, state DB size, indexing/archive configuration, hardware, sync/replay, and retention. Keep optional indexing and archival services separate from minimum validation-node requirements.

No mainnet synchronization or bulk download was attempted. WO01–WO04 artifacts remain unchanged. The new query utility is bounded to small JSON-RPC responses and does not retrieve blocks.
