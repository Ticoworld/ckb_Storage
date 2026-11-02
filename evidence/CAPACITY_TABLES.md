# Generated capacity model

All totals below include newly required experimental code, and exclude already deployed system code.

Native balance is counted once, inside cell capacity. These are formula extrapolations, not population tests.

| Users | Standard / existing 1-of-2, 1 CKB balance | Policy, 1 CKB balance | Pool, 1 CKB balance (exit fails for N>1) | Embedded key per user, 1 CKB balance |
|---:|---:|---:|---:|---:|
| 1 | 61 | 52,694 | 57,262 | 44,718 |
| 10 | 610 | 53,351 | 57,271 | 447,180 |
| 1,000 | 61,000 | 125,621 | 58,261 | 44,718,000 |
| 1,000,000 | 61,000,000 | 73,052,621 | 1,057,261 | 44,718,000,000 |

| Users | Standard / existing 1-of-2, 100 CKB balance | Policy, 100 CKB balance | Pool, 100 CKB balance |
|---:|---:|---:|---:|
| 1 | 100 | 52,721 | 57,289 |
| 10 | 1,000 | 53,621 | 58,189 |
| 1,000 | 100,000 | 152,621 | 157,189 |
| 1,000,000 | 100,000,000 | 100,052,621 | 100,057,189 |

Existing infrastructure: sighash+data = 1,100,746 CKB; multisig+data = 1,101,122 CKB by occupied-byte calculation. These are not new mainnet user deposits and are not summed twice when shared.

Embedded-code extrapolation assumes each independently keyed binary has the measured representative size; only one such identity was compiled.

The CSV includes $0.01, $1 and $10/CKB, full relevant infrastructure, per-user amortization and security qualifications.
