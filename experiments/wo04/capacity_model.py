#!/usr/bin/env python3
"""Translate measured CKB cell-column bytes into transparent capacity scenarios.

No operating-cost or safe-network-capacity estimates are fabricated. Database
byte projections are linear extrapolations from the 100k fixture, marked as such.
"""
import csv, json, math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DATA = ROOT / "evidence/wo04"
OUTPUT_COUNTS = [1_000_000, 100_000_000, 1_000_000_000]
PRICE_USD = [0.01, 1.0, 10.0]
BYTES_PER_CKB = [1, 2, 4, 8, 16, 61]
OUTPUT_OCCUPIED_BYTES = 61

def main():
    samples = []
    for name in ("store-fixture.csv", "store-fixture-cache0.csv"):
        with (DATA / name).open(newline="") as f:
            for row in csv.DictReader(f):
                if int(row["live_cells"]) == 100_000:
                    samples.append({"source": name, "churn_rounds": int(row["churn_rounds"]),
                                    "cache_compacted_live_bytes_per_cell":
                                    (int(row["db_bytes_after_live_compaction"]) - int(row["empty_db_bytes"])) / 100_000})
    measured_min = min(x["cache_compacted_live_bytes_per_cell"] for x in samples)
    measured_max = max(x["cache_compacted_live_bytes_per_cell"] for x in samples)
    rows=[]
    for count in OUTPUT_COUNTS:
        for price in PRICE_USD:
            for bpc in BYTES_PER_CKB:
                capacity = math.ceil(OUTPUT_OCCUPIED_BYTES / bpc)
                rows.append({
                    "users_outputs": count,
                    "ckb_price_usd": price,
                    "bytes_per_ckb_protocol_scenario": bpc,
                    "minimum_output_capacity_ckb_including_1ckb_principal": capacity,
                    "additional_recoverable_capacity_ckb_per_user": max(0, capacity - 1),
                    "principal_ckb_per_user": 1,
                    "additional_capacity_usd_per_user": max(0, capacity - 1) * price,
                    "aggregate_additional_capacity_ckb": max(0, capacity - 1) * count,
                    "aggregate_additional_capacity_usd": max(0, capacity - 1) * count * price,
                    "fresh_output_occupancy_bytes_per_user": OUTPUT_OCCUPIED_BYTES,
                    "same_61ckb_capacity_output_count_ratio": 61 / capacity,
                    "measured_compacted_cell_db_bytes_per_user_low_high": [measured_min, measured_max],
                    "linear_projected_compacted_cell_db_bytes_low_high": [measured_min * count, measured_max * count],
                    "measured_data_scope": "CKB cell, cell_data, cell_data_hash columns; no block body/indexer/archive",
                    "estimated_usd_storage_rent": None,
                    "safe_network_admission_count": None,
                    "current_legacy_total_supply_ckb": None,
                    "legacy_capacity_entitlement_ckb": None,
                    "status": "capacity arithmetic measured; byte projections extrapolated; no safe-capacity claim"
                })
    model={
        "method":"linear byte extrapolation from 100,000 synthetic no-data CKB CellEntry rows, 61 occupied bytes each",
        "measured_samples":samples,
        "measured_post_compaction_cell_db_bytes_per_cell_range": [measured_min, measured_max],
        "scenario_count":len(rows),
        "scenarios":rows,
        "unavailable_inputs":["full-node live-cell count", "historical database footprint", "indexer footprint", "per-node hardware/resource cost", "network safe byte budget", "current total supply and outstanding legacy capacity exercise"],
        "prohibited_proxy_note":"WO03's 45,000,000,000 CKB entitlement placeholder is not used; no source verified that as current mainnet supply."
    }
    (DATA/"capacity-scenarios.json").write_text(json.dumps(model,indent=2,allow_nan=False)+"\n")
    with (DATA/"capacity-scenarios.csv").open("w",newline="") as f:
        w=csv.DictWriter(f,fieldnames=list(rows[0]),lineterminator="\n");w.writeheader()
        for r in rows:
            rr=r.copy(); rr["measured_compacted_cell_db_bytes_per_user_low_high"]=json.dumps(rr["measured_compacted_cell_db_bytes_per_user_low_high"])
            rr["linear_projected_compacted_cell_db_bytes_low_high"]=json.dumps(rr["linear_projected_compacted_cell_db_bytes_low_high"])
            w.writerow(rr)
    assert len(rows)==54
    assert min(x["cache_compacted_live_bytes_per_cell"] for x in samples)>150
    assert max(x["cache_compacted_live_bytes_per_cell"] for x in samples)<165
    assert math.ceil(61/1)==61 and math.ceil(61/61)==1
    print(f"wrote {len(rows)} capacity scenarios; measured compacted cell DB footprint {measured_min:.2f}-{measured_max:.2f} bytes/cell")

if __name__=="__main__": main()
