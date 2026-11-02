#!/usr/bin/env python3
"""Capture small CKB mainnet JSON-RPC header responses (no blocks or sync).

Usage:
  python3 experiments/wo04-r1/query_rpc.py --out evidence/wo04-r1/rpc \
    --endpoint https://mainnet.ckb.dev --endpoint https://mainnet.ckbapp.dev

Optionally pass --height N for each historical header checkpoint. Each RPC
response is retained verbatim with request metadata and SHA-256 in manifest.json.
"""
import argparse
import datetime as dt
import hashlib
import json
import pathlib
import urllib.request


def rpc(url, method, params):
    body = json.dumps({"id": 1, "jsonrpc": "2.0", "method": method,
                       "params": params}).encode()
    request = urllib.request.Request(url, data=body,
                                     headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(request, timeout=15) as response:
        raw = response.read(64 * 1024)
        if response.read(1):
            raise RuntimeError("response exceeds 64 KiB safety bound")
    decoded = json.loads(raw)
    if "error" in decoded:
        raise RuntimeError(f"JSON-RPC error: {decoded['error']}")
    return raw, decoded["result"]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--endpoint", action="append", required=True)
    parser.add_argument("--height", action="append", default=[],
                        help="historical block number in decimal or 0x hex; repeatable")
    parser.add_argument("--out", type=pathlib.Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    manifest = {"captured_utc": dt.datetime.now(dt.timezone.utc).isoformat(),
                "method_scope": ["get_tip_header", "get_blockchain_info",
                                 "get_header_by_number"],
                "endpoints": []}
    heights = [int(h, 0) for h in args.height]
    for index, endpoint in enumerate(args.endpoint):
        entry = {"endpoint": endpoint, "responses": []}
        calls = [("get_tip_header", []), ("get_blockchain_info", [])]
        calls.extend(("get_header_by_number", [hex(height)]) for height in heights)
        for seq, (method, params) in enumerate(calls):
            try:
                raw, result = rpc(endpoint, method, params)
                name = f"provider-{index+1}-{seq+1}-{method}.json"
                (args.out / name).write_bytes(raw + b"\n")
                entry["responses"].append({"method": method, "params": params,
                    "file": name, "sha256": hashlib.sha256(raw).hexdigest(),
                    "result": result})
            except Exception as exc:  # retain the failure without fabricating a value
                entry["responses"].append({"method": method, "params": params,
                                           "error": f"{type(exc).__name__}: {exc}"})
        manifest["endpoints"].append(entry)
    (args.out / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(json.dumps({"out": str(args.out), "providers": len(args.endpoint),
                      "responses": sum(len(x["responses"]) for x in manifest["endpoints"]),
                      "manifest": str(args.out / "manifest.json")}, indent=2))


if __name__ == "__main__":
    main()
