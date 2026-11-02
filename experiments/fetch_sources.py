"""Fetch pinned sources alongside the checkout; never reset an existing checkout."""
import json
import pathlib
import subprocess
root = pathlib.Path(__file__).resolve().parents[1]
cache = root.parent / 'wo01-sources'
cache.mkdir(exist_ok=True)
for s in json.loads((root / 'evidence/sources.json').read_text())['sources']:
    dest = cache / s['name']
    if not dest.exists():
        subprocess.run(['git', 'init', str(dest)], check=True)
        subprocess.run(['git', '-C', str(dest), 'remote', 'add', 'origin', s['url']], check=True)
        subprocess.run(['git', '-C', str(dest), 'fetch', '--depth', '1', 'origin', s['commit']], check=True)
        subprocess.run(['git', '-C', str(dest), 'checkout', '--detach', s['commit']], check=True)
    head = subprocess.check_output(['git', '-C', str(dest), 'rev-parse', 'HEAD'], text=True).strip()
    if head != s['commit']:
        raise SystemExit(f'{dest}: wrong revision; preserve existing work and use a separate cache')
    if subprocess.check_output(['git', '-C', str(dest), 'status', '--porcelain'], text=True):
        raise SystemExit(f'{dest}: modified source cache; inspect before using')
    print(s['name'], head)
