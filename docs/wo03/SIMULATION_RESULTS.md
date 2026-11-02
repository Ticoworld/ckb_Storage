# Simulation results and reproduction

## What was modeled

Run from the repository root:

```sh
python3 experiments/wo03/model.py
```

The standard-library Python model emits 1,215 rows across five mechanisms, three starting CKB prices (`$0.01 / $1 / $10`), three durations (`1y / 10y / indefinite`), three demand cases (1m / 100m / 1bn users), three resource-cost multipliers (0.9 / 1.0 / 1.5), and three token paths (stable / 90% crash / 99% crash). It writes `evidence/wo03/model-results.csv`, `model-results.json`, and the expiry counterexample in `lease-liveness.json`. Assertions check the principal/collateral split, the 61x conversion condition, price-crash security-fee scaling, and expiry rights trade-off.

The demand cases are simultaneous occupied-state snapshots, not throughput or forecasts. At 61 logical bytes per user they represent 0.061 GB, 6.1 GB and 61 GB of live output bytes. A 12x replication equivalent and `$0.00001 / logical byte-year` resource-cost assumption imply annual physical burden of `$7,320 / $732,000 / $7.32m` respectively before other state/index/archive costs. These assumptions are deliberately editable and are not asserted CKB node measurements.

## Direct affordability

For a new standard-shaped cell holding 1 CKB:

| CKB price | Principal | Extra refundable capacity (60 CKB) | Total locked (61 CKB) | 5% annual opportunity cost on extra collateral | Fee in fixture |
|---:|---:|---:|---:|---:|---:|
| $0.01 | $0.01 | $0.60 | $0.61 | $0.03 | 0.0001 CKB |
| $1 | $1 | $60 | $61 | $3 | 0.0001 CKB |
| $10 | $10 | $600 | $610 | $30 | 0.0001 CKB |

These totals separate refundable capital from consumed transaction fee. A 1-year or 10-year user does not pay the full 60 CKB as a fee; they retain the cell's capacity if they retain spend control. On indefinite tenure the principal and collateral remain locked; at a 5% simple opportunity-rate benchmark the foregone return is not bounded. A 90%/99% CKB price crash leaves nominal 60 CKB collateral unchanged but reduces its USD mark and reduces USD-denominated fee revenue by the same price multiplier in the model.

## Alternative sensitivities

- **61-byte/CKB conversion:** Removes 60 CKB of *capacity* from the new user's required output; total output principal remains 1 CKB and the output still stores 61 bytes. In the fixed user scenario it does not increase bytes. It allows up to 61 times more such outputs per same amount of capacity-denominated CKB, unless an independent byte cap admits fewer users. The model does not assert that a node budget supports that induced upper bound.
- **Term rent:** At the illustrative resource assumption, one 61-byte output rents for `$0.00732/year` after 12x replication; nominal 10-year rent is `$0.0732` and flat-cost perpetuity discounted at 3% is `$0.244`. Those values are only affordable if the cost assumption is close to reality and users can pay in a reliable unit. At lease expiry, deletion loses independent claim enforcement; retaining anti-replay state/data means rent does not end for that portion.
- **Prepaid endowment:** At the same assumptions, it requires `$0.00732` for 1 year, `$0.06244` present value for 10 annual charges discounted at 3%, and `$0.244` for a constant-cost perpetuity discounted at 3%. Discount rate 0% makes indefinite funding infinite. An external USD-equivalent endowment needs external assets, custody and price/return assumptions; CKB-denominated funds retain token-crash risk. The finite PV is not a protocol-supplied yield.
- **Market-priced credits:** A deliberately hostile scarcity stress prices each byte at `1 CKB/byte × 10` at 10% spare credits. A 61-byte output then needs 610 CKB of additional credit collateral, plus 1 CKB monetary principal. This is not a market-clearing estimate; it falsifies any claim that separate token units are costless or immune to scarcity.
- **Demand/fees/security:** At 0.0001 CKB per operation, 1m operations cover `$1 / $100 / $1,000` at the three starting token prices, before crash. The explicit annual security stress ranges `$1m / $10m / $100m`; fees alone fail all but the most extreme high-volume/high-price row. Current issuance and other revenue are not calibrated here, so this does not establish current security adequacy or a fork's required budget.

## Feasible ranges and limitations

For an indefinite prepaid fund, present value is finite only when expected real return is positive. With a constant annual real cost `c`, the simplified perpetuity is `c/r`; as `r → 0+`, required capital diverges, and for a nonpositive net return it is unbounded. For finite tenure `T`, discounted cost is `sum(c_t/(1+r)^t)`. Hardware costs, replication, rent price, CKB/USD, and security budgets are independent inputs, not derived from market observations. The full ranges are in the JSON/CSV, including 10x sybil demand and price crashes.

This model omits endogenous equilibrium: it does not predict rent price, resource efficiency, CKB adoption, miner hashpower, hardware purchase response, or user willingness to pay. It cannot prove solvency or equilibrium. It provides conditional arithmetic and counterexamples. The largest unknown for deciding whether a bounded, rented model can be sustainable yet affordable is an independently measured lower/upper bound on long-run marginal resource cost **and** the cost of maintaining spendable anti-replay state for offline claims. See [decision](DECISION.md).
