"""Check evidence coherence/provenance after executing the real verifier workflow."""
import csv
import hashlib
import json
import pathlib
import re

root = pathlib.Path(__file__).resolve().parents[1]
r = json.loads((root/'evidence/validator-results.json').read_text())
cases = r['cases']
assert len(cases) == 72
assert len({c['name'] for c in cases}) == len(cases)
assert all(c['test_passed'] and c['accepted'] == c['expected_accept'] for c in cases)
assert r['proof_refresh_host_comparisons'] == 240
for dep in r['deployment']:
    p = root/'experiments/bin'/dep['contract']
    if p.exists():
        b = p.read_bytes()
        assert len(b) == dep['elf_bytes']
        assert hashlib.blake2b(b, digest_size=32, person=b'ckb-default-hash').hexdigest() == dep['code_data_hash']
        assert len(b) + 61 == dep['deployment_occupied_ckb_with_20_byte_lock']
for record in json.loads((root/'evidence/source-files.json').read_text()):
    p = root.parent/'wo01-sources'/record['source']/record['path']
    assert hashlib.sha256(p.read_bytes()).hexdigest() == record['sha256'], str(p)
rows = list(csv.DictReader((root/'evidence/capacity-model.csv').open()))
assert len(rows) == 120
for row in rows:
    cell, code, system = (int(row[k]) for k in ['live_user_cells_capacity_ckb','new_code_capacity_ckb','existing_relevant_system_capacity_ckb'])
    assert int(row['incremental_live_capacity_ckb']) == cell + code
    assert int(row['full_capacity_including_relevant_system_code_ckb']) == cell + code + system
for doc in [root/'README.md', *sorted((root/'docs').glob('*.md')), root/'experiments/README.md']:
    for link in re.findall(r'\]\(([^)]+)\)', doc.read_text()):
        if '://' in link or link.startswith('#'):
            continue
        assert (doc.parent/link.split('#')[0]).exists(), (doc,link)
for p in ['WO01_CONSENSUS_BOUNDARIES.md','WO01_FIRST_PRINCIPLES_QUESTIONS.md','WO01_EXISTING_APPROACHES.md','WO01_HYPOTHESIS_REGISTER.md','WO01_CAPACITY_ECONOMICS.md','WO01_EXPERIMENT_RESULTS.md','WO01_BREAKTHROUGH_DECISION.md']:
    assert (root/'docs'/p).stat().st_size > 0
toolchain = json.loads((root/'evidence/toolchain.json').read_text())
for path, expected in toolchain['source_sha256'].items():
    assert hashlib.sha256((root/path).read_bytes()).hexdigest() == expected, path
raw = (root/'evidence/validator-run.txt').read_text()
assert 'ALL 72 CASES PASSED' in raw
for case in cases:
    assert 'PASS ' + case['name'] + ' ' in raw, case['name']
print('Evidence coherent: 72 case outcomes, 240 refresh comparisons, binaries, source hashes, 120 economic scenarios and document links')
