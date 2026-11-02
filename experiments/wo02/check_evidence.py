"""Validate evidence coherence and preserve every file from the verified WO01 base."""
import csv
import hashlib
import json
import pathlib
import re
import subprocess
from decimal import Decimal

root=pathlib.Path(__file__).resolve().parents[2]
base='ac79f830c896b8b431de9ce869df0883ffc416d5'
listing=subprocess.check_output(['git','ls-tree','-rz',base],cwd=root).split(b'\0')
protected=0
for entry in listing:
    if not entry:
        continue
    meta,path=entry.split(b'\t',1);mode,kind,expected=meta.split()
    p=root/path.decode();assert p.is_file(),p
    actual=subprocess.check_output(['git','hash-object','--no-filters',str(p)],cwd=root).strip()
    assert actual==expected, f'WO01 changed: {p}'
    assert bool(p.stat().st_mode & 0o111)==(mode==b'100755'),f'WO01 mode changed: {p}'
    protected+=1
r=json.loads((root/'evidence/wo02/validator-results.json').read_text())
cases=r['cases'];assert len(cases)==142 and len({c['name'] for c in cases})==142
assert sum(c['accepted'] for c in cases)==110
assert all(c['test_passed'] and c['accepted']==c['expected_accept'] for c in cases)
assert len(r['exhaustive_orders'])==24
assert len({tuple(o['order']) for o in r['exhaustive_orders']})==24
assert all(o['remaining_liabilities_shannons']==0 and o['all_claims_paid'] for o in r['exhaustive_orders'])
assert r['public_proof_refreshes']==144
assert len(r['independent_client_archive_attacks'])==4
assert all(a['rejected'] for a in r['independent_client_archive_attacks'])
for filename,expected,bytes_count in [('sum_type',r['type_code_data_hash'],r['type_elf_bytes']),('anyone_can_pay',r['acp_data_hash'],r['acp_elf_bytes'])]:
    b=(root/'experiments/wo02/bin'/filename).read_bytes()
    assert len(b)==bytes_count
    assert hashlib.blake2b(b,digest_size=32,person=b'ckb-default-hash').hexdigest()==expected
source=json.loads((root/'evidence/wo02/sources.json').read_text())
assert source['sdk']['ckb_data_hash']==source['acp_reference_expected_hash']==r['acp_data_hash']
raw=(root/'evidence/wo02/validator-run.txt').read_text()
assert 'ALL 142 CASES PASSED' in raw
assert all('PASS '+c['name']+' ' in raw for c in cases)
rows=list(csv.DictReader((root/'evidence/wo02/one-ckb-economics.csv').open()))
assert len(rows)==6
for row in rows:
    p=Decimal(row['usd_per_ckb'])
    assert Decimal(row['extra_exit_capital_usd'])==Decimal(row['extra_exit_native_capital_ckb'])*p
capital=json.loads((root/'evidence/wo02/pool-capital.json').read_text())
assert capital['new_pool_code_ckb']==r['type_elf_bytes']+41+r['always_success_bytes']+41
for a in capital['allocations']:
    assert a['live_pool_capacity_ckb']==a['claim_principal_ckb']+a['pool_reserve_ckb']
    assert a['live_pool_and_new_code_ckb']==a['live_pool_capacity_ckb']+a['new_code_ckb']
tool=json.loads((root/'evidence/wo02/toolchain.json').read_text())
for path,digest in tool['sha256'].items():
    assert hashlib.sha256((root/path).read_bytes()).hexdigest()==digest,path
for p in [root/'WO02.md',root/'experiments/wo02/README.md',*sorted((root/'docs/wo02').glob('*.md'))]:
    for link in re.findall(r'\]\(([^)]+)\)',p.read_text()):
        if '://' in link or link.startswith('#'):
            continue
        assert (p.parent/link.split('#')[0]).exists(),(p,link)
print(f'PASS: {protected} original WO01 files unchanged; 142 cases, 24 orders, 144 proof refreshes, 4 client attacks; binary/source hashes, economic arithmetic and document links coherent')
