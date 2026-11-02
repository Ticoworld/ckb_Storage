# Adversarial findings

## Best coherent candidate under attack

The best current hypothesis is **a new-format, byte-budgeted L1 monetary output with a refundable capacity floor below 61 CKB**, while preserving legacy output rights and replacing occupation-linked miner income with an explicit, sustainable security/resource fee. It changes protocol accounting; a script-only implementation cannot bypass CKB's global capacity verifier. It has not passed economics or migration tests and is not a recommended fork.

| Attack | Result | Consequence |
|---|---|---|
| 61x demand response after reducing per-output capacity from 61 CKB to 1 CKB | Each user's record is still 61 bytes, but 61x as many minimum-balance users can fit per same capacity-denominated token amount. Fixed-byte cap stops growth only by refusing some otherwise affordable users. | Affordability trades against admission and congestion rent. Need a safe protocol byte budget, not only a lower deposit. |
| 10x sybil demand (100m to 1bn outputs) | Logical live state rises 6.1 GB to 61 GB; assumed physical resource bill rises `$732k` to `$7.32m` per year. | A cheap conversion without fee/cap creates a resource subsidy; a cap/auction moves cost to users and can be captured. |
| Hardware cost +50% | Annual modeled burden scales 1.5x. | Fixed native rent loses real-cost tracking; a market rate needs governance or external index. |
| CKB price crash 90% / 99% | Native-denominated fees and issuance lose 90% / 99% USD value while fixed real storage/security obligations persist. | Rent denominated only in CKB does not stabilize purchasing power. Fiat indexing introduces price feeds/external-unit risk. |
| Indefinite state at 0% net real fund return | Required perpetuity is infinite. | “Pay once” cannot buy perpetual real service from a zero-return or devaluing fund. Positive return is not a protocol guarantee. |
| Lease lapse while owner is offline | Delete claim state and the user cannot prove remaining spendable amount/unspent status. Retain receipt plus nullifier/index and authenticated data, and persistent anti-replay state remains. | Expiry either breaks independent monetary control or retains a continuing state obligation. `lease-liveness.json` encodes these cases. |
| Censorship during renewal grace | Renewal can fail even if owner still controls the key. | A guaranteed consensus exit path/grace floor delays reclamation and must remain funded; keeper/operator is a third party. |
| Mass legacy redemption | If only occupied legacy capacity is reserved, dormant holders lose the right to create outputs. If all hypothetical legacy capacity is reserved, it can crowd out new-format users and overwhelm any small quota. | No neutral snapshot or partial reserve was identified. Maximum right and maximum feasible node budget must be reconciled by a rule that openly changes entitlements or spends real reserved resources. |
| Storage-credit price at 10% headroom | Stress rule makes a 61-byte output's extra credit collateral 610 CKB. | Splitting assets creates a scarce collateral market; it may cost more than current capacity. This is a scenario stress, not price prediction. |
| Archive/data provider disappears | A commitment without available witness data cannot reconstruct the claim; keeping an exit receipt requires an archive/data-availability path. | No off-chain receipt is counted as self-custodial unless its data and replay state remain independently available. |
| Miners and nodes paid from same fee | A dollar assigned to storage operators cannot also secure mining unless one is explicitly unpaid or a separate income source exists. | Must specify fee split, issuance, and stress security floor; model does not count fees twice. |
| Higher rent after users form state | Sudden parameter change can render existing positions underfunded or force exit. | Transition protections either reserve old rates, subsidize the gap, or impose retrospective cost; each bears an explicit burden. |

## Classification against protocol requirements

- **Current consensus:** No protocol-level waiver of the 61 CKB measured standard output. Existing applications can represent shared/off-chain claims or use ACP, but that changes the target guarantee.
- **Application layer:** A program can batch state or issue proofs; it cannot make an L1 output pass the global occupied-capacity verifier below its rule.
- **Soft/hard fork:** Changing capacity per byte, rent, expiry, storage fund, or allowed output classes requires a consensus change and activation/migration rules. Treat as hard-fork-level economic semantics for planning; this report does not assess exact versioning feasibility.
- **Issuance/DAO:** If occupied capacity no longer drives same counted-capacity and secondary-issuance rules, DAO exchange rate, miners' state income and treasury accounting need redesign. A consensus change that leaves old accounting untouched risks under/overpayment.
- **Legacy rights:** Full preservation of old redemption rights requires unspent reserve or a consensus priority claim, not an airdrop formula alone. Unoccupied future rights cannot be reconstructed from occupied cell snapshots.
- **Third-party capital:** USD rent/endowment uses external funding/custody or an oracle; stable rent from a provider is a counterparty. A native storage credit is a scarce asset and collateral. A loan finances collateral but does not erase principal or risk.
- **Security/validation:** None of the candidate economics was run against a forked validator; no fork code or consensus experiment was authorized. The model does not prove miner equilibrium or node decentralization.

## Stop conditions

Stop if a measured safe byte budget cannot support the legacy worst-case reserve plus affordable new demand, or if the fee/rent needed to meet a documented security floor exceeds the affordable amount for a 1 CKB holder. Also stop if offline users need perpetual anti-replay state whose total cost returns to the original capacity floor. These are falsifiable conditions; current assumptions are too weak to claim they have already been measured.
