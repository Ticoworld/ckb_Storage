# WO04-R1: DAO state facts and live-data access result

## Result

The DAO header gives a consensus-derived series for occupied capacity (`U_i`) and total issued capacity (`C_i`). It can establish the chain's change in occupied-capacity rights over time, but not the physical database bytes consumed by a node. This execution could not obtain live headers: the managed HTTP policy is restricted and omits both `mainnet.ckb.dev` and `mainnet.ckbapp.dev`; requests to each failed at the outbound CONNECT proxy with HTTP 403. No mainnet sync or bulk download was attempted. We therefore report no current `U_i`, `C_i`, growth rate, live-cell count, or chain database size.

**Decision: RESEARCH OPEN.** Real DAO header observations would improve the demand side of WO04, but operator measurements remain necessary before a credible global storage-cost or safe-conversion range can be established. `U_i` is denominated in CKB-capacity (occupied bytes at current consensus rules), not physical RocksDB bytes. The earlier 155–160 database bytes per 61-byte fixture cell only measures the specified fixture columns and remains an incomplete microbenchmark.

## Exact DAO interpretation

RFC 0023, “Deposit and Withdraw in Nervos DAO,” §Calculation, defines the 32-byte `dao` field as four unsigned 64-bit little-endian values, in order:

| Bytes | Field | Meaning |
|---|---|---|
| 0–7 | `C_i` | Total issuance through block `i` |
| 8–15 | `AR_i` | Accumulated rate, scaled by `10^16` |
| 16–23 | `S_i` | Unissued secondary issuance, including unclaimed DAO compensation and treasury funds |
| 24–31 | `U_i` | Occupied capacities of all cells through block `i` |

`C_i` is total issuance, not circulating supply. It includes CKB issued under consensus even when capacity is locked in Nervos DAO or treasury cells. `U_i` is an aggregate capacity quantity, not cell count or bytes on disk. RFC 0023 defines `U_0 = U_out,0 - U_in,0` and later `U_i = U_(i-1) + U_out,i - U_in,i`.

The same order and endian encoding are implemented by `extract_dao_data` and `pack_dao_data` in pinned CKB source `a4aa1286a45bc1d42c30f6698dc51521c3749b40`, `util/dao/utils/src/lib.rs`; current DAO accounting subtracts freed occupied capacity and adds new occupied capacity in `util/dao/src/lib.rs`. This independently corroborates the RFC layout.

## Genesis-cell exception

The pinned source sets `SATOSHI_CELL_OCCUPIED_RATIO = 6/10` and identifies the Satoshi public-key hash in `spec/src/consensus.rs`. Genesis DAO construction applies that ratio to the Satoshi gift output when deriving `U_0`; `modified_occupied_capacity` applies the same rule on the input side when that genesis cell is spent. This avoids treating that special genesis output as an ordinary cell for DAO occupied-capacity accounting. It is not a permanent subtraction from all later `U_i`, nor a general adjustment to subtract from current occupied capacity. For all other cells, ordinary occupied capacity applies.

Whether the particular Satoshi gift output remains unspent at the current tip cannot be established from the local source checkout. If it has been spent, the consensus code subtracts its special 60%-of-capacity amount in that spending transition; if not, that historical amount remains represented in the running `U_i`. A live-cell query or operator state is required to verify the current status. Do not subtract its genesis occupancy again from a decoded tip `U_i`.

## RPC attempt and reproduction

At capture time, both read-only `get_tip_header` requests failed before reaching the HTTP service:

| Host | Result |
|---|---|
| `mainnet.ckb.dev` | outbound CONNECT proxy returned HTTP 403 |
| `mainnet.ckbapp.dev` | outbound CONNECT proxy returned HTTP 403 |

The current managed environment reports restricted HTTP access; neither host appears in its allowed-host set. Thus there are no endpoint responses to compare and no claim that these endpoints are independent operators. To avoid fabricating evidence, the query results and all decoded live values are explicitly absent.

Once access is enabled, run the bounded stdlib-only capture utility:

```sh
python3 experiments/wo04-r1/query_rpc.py \
  --out evidence/wo04-r1/rpc \
  --endpoint https://mainnet.ckb.dev \
  --endpoint https://mainnet.ckbapp.dev \
  --height 0x0
```

The tool requests only `get_tip_header`, `get_blockchain_info`, and explicitly requested `get_header_by_number` calls. It caps response bodies at 64 KiB, records raw JSON with SHA-256 hashes, and retains per-call failures. Confirm operator/provider independence before treating matching hosts as independent corroboration. Use matched header hashes at each checkpoint before comparing DAO fields. Annual growth should be calculated from observed header timestamps, not guessed block-rate assumptions.

## What chain headers can and cannot establish

With verified tip and historical headers, `U_i` differences provide actual growth in consensus-counted occupied capacity, and `C_i` provides actual cumulative issuance at those blocks. The four packed values also expose DAO accounting state (`AR_i`, `S_i`). This can replace fabricated or stale supply/occupancy proxies and constrain an economic model's chain-state demand inputs.

Header data alone cannot establish:

- current count of live cells (the DAO field stores aggregate occupied capacity, not cell count);
- current full-node disk usage, compacted RocksDB amplification, or per-cell metadata/index overhead;
- optional indexer, freezer/archive, pruning, and historical block-data sizes;
- memory, CPU, synchronization/replay time, deletion cost, or required operator bandwidth;
- whether a particular endpoint's reported state includes the same indexing and retention services as another.

Those require reproducible operator snapshots or controlled full-node measurements against pinned software, hardware, database options, and retention/indexing modes. A full node must independently validate consensus and maintain enough chain/state data to do so; indexers, archival/history services, and application-facing search layers may be separately operated but their resource costs should not be conflated with the minimal validation-node burden. To bound affordable storage pricing safely, obtain both header-derived occupied-capacity growth and independently reproducible node resource observations.

## Sources and provenance

- RFC repository commit `0c945451c24194f4d34d81195c0159df0a67581a`; [RFC 0023](https://github.com/nervosnetwork/rfcs/blob/0c945451c24194f4d34d81195c0159df0a67581a/rfcs/0023-dao-deposit-withdraw/0023-dao-deposit-withdraw.md), §Calculation.
- CKB source commit `a4aa1286a45bc1d42c30f6698dc51521c3749b40` (release v0.210.0): `util/dao/utils/src/lib.rs`, `util/dao/src/lib.rs`, `spec/src/consensus.rs`, and `verification/src/genesis_verifier.rs`.
- WO04 fixture limits: `docs/wo04/DECISION.md` and `experiments/wo04` (preserved unchanged).
- RPC capture code: [`query_rpc.py`](../../experiments/wo04-r1/query_rpc.py). No live-chain response was captured in this environment.
