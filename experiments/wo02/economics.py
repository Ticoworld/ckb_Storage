"""WO02 capital/fee comparisons, explicitly labeled arithmetic extrapolations."""
import csv
import json
from pathlib import Path
root=Path(__file__).resolve().parents[2]
r=json.loads((root/'evidence/wo02/validator-results.json').read_text())
fee=r['fixture_fee_shannons']/100_000_000
new_code=r['type_elf_bytes']+41+r['always_success_bytes']+41
system=r['acp_deployment_occupied_ckb_standard_lock']+r['standard_elf_bytes']+61+r['secp_data_bytes']+61
rows=[]
for price in [1,10]:
    # A fresh independently spendable 61 CKB cell with a gross 1 CKB claim
    # needs 60+fee extra native tokens. Funding cannot be invented by a root.
    for route,owned,extra,preexisting,exitfee,dep in [
        ('fresh_standard',1,60+fee,0,fee,'not executable by minimal pool; requires extra capital/protocol integration'),
        ('existing_acp',1,0,61,fee,'actual ACP accepts; recipient key needed only for later independent spend'),
        ('continued_shared_claim',1,0,0,0,'conditional local proof/public history, code and inclusion; no general payment implemented'),
    ]:
        rows.append(dict(usd_per_ckb=price,route=route,claim_ckb=owned,
            extra_exit_native_capital_ckb=extra,preexisting_recipient_capacity_ckb=preexisting,
            measured_exit_fee_ckb=exitfee,claim_plus_extra_ckb=owned+extra,
            claim_usd=price*owned,extra_exit_capital_usd=price*extra,
            preexisting_recipient_usd=price*preexisting,exit_fee_usd=price*exitfee,
            pool_reserve_ckb=147,new_pool_code_ckb=new_code,
            relevant_shared_system_code_ckb=system,
            qualification=dep))
with (root/'evidence/wo02/one-ckb-economics.csv').open('w',newline='') as f:
    w=csv.DictWriter(f,fieldnames=rows[0].keys(),lineterminator="\n");w.writeheader();w.writerows(rows)
alloc=[]
for n in [1,4,256]:
    # Equal 1 CKB claims; N=256 admission actually executed, others scenario arithmetic.
    alloc.append(dict(claims=n,claim_principal_ckb=n,pool_reserve_ckb=147,
        new_code_ckb=new_code,live_pool_capacity_ckb=n+147,
        live_pool_and_new_code_ckb=n+147+new_code,
        per_claim_live_capacity_excluding_new_code_ckb=1+147/n,
        per_claim_including_new_code_ckb=(n+147+new_code)/n,
        existing_acp_cells_if_all_exit_ckb=61*n,
        scope='allocation model, no coordinated multi-pool design or payment capability inferred'))
(root/'evidence/wo02/pool-capital.json').write_text(json.dumps({'allocations':alloc,'relevant_shared_system_code_ckb':system,'new_pool_code_ckb':new_code,'new_code_control':'immutable zero-code-hash burn-lock code cells; this capital is irrecoverable; 20-byte reclaimable-owner-lock alternative costs 40 CKB more but introduces removal risk', 'note':'shared system secp/ACP code occupied minima, not actual configured chain holdings; do not count again per user; no live deployment verified'},indent=2)+'\n')
selected=['admit_fully_backed_four_independent_claims','one_ckb_exit_existing_real_acp','existing_acp_direct_one_ckb_deposit','existing_acp_direct_same_one_ckb_gross_budget','acp_owner_independently_spends_credited_cell','operator_gone_separate_client_reconstructed_exit','admission_256_actual_independent_keys']
transactions=[c for c in r['cases'] if c['name'] in selected]
(root/'evidence/wo02/measured-transactions.json').write_text(json.dumps(transactions,indent=2)+'\n')
print(f'Generated {len(rows)} price/exit scenarios and {len(alloc)} pool capital allocations; new code={new_code} CKB, relevant shared system code={system} CKB')
