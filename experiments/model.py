"""Economic extrapolations from validator-measured occupied capacities and ELF sizes.
No full-population execution or market/throughput forecast is implied.
"""
import csv
import json
import math
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
r = json.loads((ROOT / 'evidence/validator-results.json').read_text())
d = {x['contract']: x['deployment_occupied_ckb_with_20_byte_lock'] for x in r['deployment']}
secp_infra = d['standard_secp'] + d['standard_secp_data']
multi_infra = d['standard_multisig'] + d['standard_secp_data']
rows = []
for n in [1, 10, 1000, 1_000_000]:
    for balance in [1, 100]:
        # Reserve needed so the LAST remaining claim still underwrites a live root
        # until closure. For the equal 100 CKB case it is zero (executed at N=4).
        reserve = max(0, 73 - balance)
        variants = [
            ('standard_owner', n * max(61, balance), 0, secp_infra, True, 'no independent recovery'),
            ('existing_multisig_1of2', n * max(61, balance), 0, multi_infra, True, 'two equivalent spend keys; no delay/global revocation'),
            ('custom_policy', n * max(73, balance), d['policy'], 0, True, 'same narrow two-key authority model'),
            ('shared_pool', n * balance + reserve, d['pool'], secp_infra, balance >= 61.00001 or n == 1, 'immutable claims; fresh outputs; cached proofs and public witness history'),
            ('embedded_key', n * max(41, balance), n * d['embedded'], 0, True, 'one code deployment per independent key; equal-size-code extrapolation'),
        ]
        for name, cells, new_code, existing_code, exit_ok, note in variants:
            for price in [0.01, 1, 10]:
                rows.append(dict(users=n, native_balance_ckb_each=balance, architecture=name,
                    live_user_cells_capacity_ckb=cells,
                    new_code_capacity_ckb=new_code, existing_relevant_system_capacity_ckb=existing_code,
                    incremental_live_capacity_ckb=cells+new_code,
                    full_capacity_including_relevant_system_code_ckb=cells+new_code+existing_code,
                    incremental_ckb_per_user=(cells+new_code)/n,
                    usd_per_ckb=price, incremental_capital_usd=(cells+new_code)*price,
                    independent_fresh_l1_exit_funded=exit_ok,
                    evidence_kind='formula extrapolation; see actual cases and proof-depth measurements', notes=note))
with (ROOT/'evidence/capacity-model.csv').open('w',newline='') as f:
    w=csv.DictWriter(f,fieldnames=rows[0].keys());w.writeheader();w.writerows(rows)
# Per-exit fee-rate illustrations are not tested mempool admission prices.
fees=[]
for m in r['proof_depth_measurements']:
    for rate in [1000,10000]:
        fees.append(dict(logical_users=m['logical_users'],serialized_bytes=m['serialized_transaction_bytes'],
            assumed_shannons_per_kb=rate,byte_only_fee_shannons=math.ceil(m['serialized_transaction_bytes']*rate/1000),
            cycles=m['cycles'],qualification='illustrative bytes-only rate; excludes pool policy and cycle-based virtual size'))
(ROOT/'evidence/fee-scenarios.json').write_text(json.dumps(fees,indent=2)+'\n')
summary=['# Generated capacity model\n',
    'All totals below include newly required experimental code, and exclude already deployed system code.\n',
    'Native balance is counted once, inside cell capacity. These are formula extrapolations, not population tests.\n',
    '| Users | Standard / existing 1-of-2, 1 CKB balance | Policy, 1 CKB balance | Pool, 1 CKB balance (exit fails for N>1) | Embedded key per user, 1 CKB balance |',
    '|---:|---:|---:|---:|---:|']
for n in [1,10,1000,1_000_000]:
    summary.append(f'| {n:,} | {61*n:,} | {73*n+d["policy"]:,} | {n+72+d["pool"]:,} | {(41+d["embedded"])*n:,} |')
summary += ['', '| Users | Standard / existing 1-of-2, 100 CKB balance | Policy, 100 CKB balance | Pool, 100 CKB balance |', '|---:|---:|---:|---:|']
for n in [1,10,1000,1_000_000]:
    summary.append(f'| {n:,} | {100*n:,} | {100*n+d["policy"]:,} | {100*n+d["pool"]:,} |')
summary += ['',f'Existing infrastructure: sighash+data = {secp_infra:,} CKB; multisig+data = {multi_infra:,} CKB by occupied-byte calculation. These are not new mainnet user deposits and are not summed twice when shared.', '',
    'Embedded-code extrapolation assumes each independently keyed binary has the measured representative size; only one such identity was compiled.', '',
    'The CSV includes $0.01, $1 and $10/CKB, full relevant infrastructure, per-user amortization and security qualifications.']
(ROOT/'evidence/CAPACITY_TABLES.md').write_text('\n'.join(summary)+'\n')
print(f'Generated {len(rows)} economic scenarios and {len(fees)} illustrative fee scenarios')
