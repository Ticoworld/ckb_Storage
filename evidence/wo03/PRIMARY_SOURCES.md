# WO03 primary source pins

Snapshot captured 2026-11-02. These are immutable source commits, not claims that every remote repository's branch tip remains identical forever. Cached file bytes and SHA-256/size records accompany this index. Where a proposal is not activated, the document says so; a proposal is not treated as current consensus.

## CKB

- CKB node `nervosnetwork/ckb`, source commit `a4aa1286a45bc1d42c30f6698dc51521c3749b40` (current HEAD checked at research time; same pin as WO01/WO02). Occupied-capacity calculation: `util/gen-types/src/extension/capacity.rs`. Global output enforcement: `verification/src/transaction_verifier.rs`, `CapacityVerifier`. DAO capacity accounting: `util/dao/src/lib.rs`, `calculate_maximum_withdraw` and counted capacity subtraction. Release tag `v0.210.0` is pinned separately in WO01 sources at `2592ddf0502cd4adfe886db893cccc866db3c60f` for a reproducible release comparison.
- RFC repository `nervosnetwork/rfcs`, commit `0c945451c24194f4d34d81195c0159df0a67581a`. RFC 0015 (Final) explains the storage/secondary-issuance/DAO rationale. RFC 0002 (Final) specifies cell capacity/occupied capacity. RFC 0019 is Withdrawn; 0022 Draft; 0026 Proposal; these are not current consensus. Full earlier WO01 source index: [`docs/WO01_SOURCES.md`](../../docs/WO01_SOURCES.md).

## Solana

- SIMD repository `solana-foundation/solana-improvement-documents`, commit `0434aab3b2c6b596f607194bee44e94088ff2591`; Solana SDK repository `anza-xyz/solana-sdk`, commit `f5fd9ece3920419edb69eaf78fb896844b3cd971`.
- Cached: SIMD-0194 proposal status **Activated** at this pin; it changes the exemption-threshold representation. SIMD-0437 incremental rent reduction and SIMD-0438 rent-increase safeguard are status **Idea**, not active. SDK `rent/src/lib.rs` contains the minimum-balance formula and checked limit behavior. Agave source commit `9db9c0263c744948de19d366350cbf23e532d2dd` has runtime `rent_collector.rs` and account storage logic; commit obtained from remote HEAD lookup. Cached raw-file manifest records hashes and any attempted paths that returned 404.

## Sui

- `MystenLabs/sui` remote HEAD commit `4f46db796a68bef95d5c6e43918426937df7721f`. Cached `crates/sui-framework/packages/sui-system/sources/storage_fund.move` documents separate rebate and nonrefundable balances, epoch processing, and charges/rebates. This is implementation evidence at the pin, not a claim that Sui's model can be transplanted into CKB unchanged.

## Cardano

- `IntersectMBO/cardano-ledger` remote HEAD commit `af08d7b551a31a3e87cf9b46dfa65bf99588294b`. Cached Alonzo `TxOut.hs` contains minimum-coin calculation tied to a protocol parameter and serialized output size. The model is an adjustable native-value floor; it still ties output admission to that native value.

## Ethereum

- EIPs repository commit `dc5d5fa688b7fe87b95a907fcc128b4fde2f4d2f`. Cached EIP-2929 (cold state access repricing), EIP-3529 (refund reduction), EIP-4444 (historical data expiry/pruning), and EIP-4844 (blob data availability/pricing). These EIPs address execution access or historical/temporary data; they do not prescribe deletion of live balances. Their statuses must be read in the pinned EIP front matter and are not generalized to all Ethereum client behavior.

Raw files for fetched external alternatives are in `evidence/wo03/primary/`. `primary-source-hashes.json` lists files copied from local checkouts with commit and SHA-256. `primary-fetched-manifest.json` records raw fetch attempts, including 404 paths, so a missing candidate file is not silently represented as a citation.

## Important measurement boundary

The source versions establish specific rules and implementations. None provides a current universal dollar cost per CKB byte-year, per-node replication factor, or socially required annual security budget. Those values are model inputs and sensitivity dimensions, not facts inferred from source code. A fork proposal would require production measurements and governance choices beyond these pins.
