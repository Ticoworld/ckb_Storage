# Legacy storage rights and migration

## What the current holder owns

Under the current CKB rule, native capacity in a live cell both represents CKB value and must cover occupied bytes. The cell owner controls spending; occupied capacity is not a separate voucher that can be redeemed independently. The DAO's counted-capacity calculation subtracts occupied capacity before applying its exchange rate. WO01's 61 CKB standard output is a measured current minimum. Existing CKB holders also retain the ability to create new outputs under the current rule while it remains in force. This protocol option is not a promise that the market price of CKB remains fixed.

If a fork makes 1 CKB cover more than 1 byte for new state, three choices arise:

1. **Keep old units fully redeemable for the old byte entitlement and give new-format units the new ratio.** This preserves old per-unit priority, but requires versioned capacity accounting. If new-format balance and old capacity are both backed by the same CKB issue/DAO assumptions, issuance and resource accounting split by class.
2. **Convert old CKB into monetary units plus storage credits.** This can be rights-neutral only if each old unit's aggregate redeemable byte claim is preserved and no credit is also sold to a new user. A holder who does not migrate must still be able to exercise old rights without a third party. Snapshot/claim windows create custody and missed-claim risks; perpetual on-chain redemption adds state and proof costs.
3. **Apply the higher byte conversion to all units.** Existing balances gain more bytes per CKB, but this is not neutral: it expands every outstanding holder's storage entitlement and state-demand potential by the multiplier. If the network cannot provision that maximum, the entitlement is nominal rather than enforceable.

## Double-counting and exhaustion cases

An airdrop of a tradable storage credit while the original CKB still buys its full current occupied capacity creates duplicate rights unless the credit is debited from an explicit reserve or the old capacity right is surrendered. A one-time snapshot based only on occupied bytes protects current occupiers but strips unoccupied holders of their future output option. A snapshot based on all CKB holdings protects the option but reserves the full legacy claim, including dormant holders, and may leave little room for new state. The honest worst case is simultaneous legacy exercise plus new demand and a native price crash.

If a reserve is finite and redemptions are first-come, first-served, the last holders can be crowded out; this changes rights and can reward transaction ordering/censorship. If redemption remains fully available, reserve all potential bytes or specify a consensus-level allocation rule that does not promise more than capacity can deliver. If conversion is optional and reversible, do not count the same capacity twice during the conversion window.

The simulator carries a clearly labeled 45 billion CKB placeholder solely as a stress input; it is not asserted to be the current circulating supply or total redeemable issuance. At current conversion this corresponds to a 45 billion byte occupied-capacity entitlement proxy. Under a 61-byte/CKB conversion for all holders it would be 2.745 trillion bytes. These are not estimates of active demand; they expose the upper-bound exercise problem. Replace the proxy with verified issuance/accounting snapshots before any protocol parameter work.

## Issuance, DAO and liquidity

RFC 0015's storage-linked secondary issuance and DAO compensation assume a capacity/occupation relationship. Decoupling that relationship requires an explicit migration of counted capacity, secondary issuance allocation, DAO deposit exchange rates and existing withdrawal calculations. Paying old DAO depositors under old rules while allocating the same issuance to a new rent/credit system risks double compensation. Removing the old state-linked miner income risks security-budget reduction. A hard fork could preserve old scripts yet change future meaning of native capacity; existing lock scripts alone do not define these global economic rights.

No migration mechanism is implemented in WO03. No consensus fork, issuance change, DAO accounting change, or deployment is proposed. An actual entitlement ledger, exact legacy balance snapshot, total reserved-byte calculation and simultaneous exercise simulation are prerequisites before an architecture could advance.
