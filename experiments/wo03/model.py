#!/usr/bin/env python3
"""Deterministic WO03 resource/capital accounting model; stdlib only.

This is a sensitivity model, not a token-price forecast or equilibrium proof.
Edit SCENARIOS/ASSUMPTIONS below; `python3 experiments/wo03/model.py` writes
machine-readable scenarios to evidence/wo03/model-results.{csv,json}.
"""
from __future__ import annotations
import csv, json, math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "evidence/wo03"
PRICES = [0.01, 1.0, 10.0]
DURATIONS = [1, 10, None]  # None means indefinite, never silently truncated.
DEMANDS = [("low", 1_000_000), ("high", 100_000_000), ("sybil_10x", 1_000_000_000)]
HARDWARE = [("decline_10pct_y", 0.90), ("flat", 1.0), ("shock_plus_50pct", 1.50)]
PRICE_PATHS = [("stable", 1.0), ("crash_90pct", 0.10), ("crash_99pct", 0.01)]
ASSUMPTIONS = {
    "standard_output_bytes": 61, "claim_ckb": 1.0, "fixture_fee_ckb": 0.0001,
    "annual_opportunity_rate": 0.05, "annual_real_discount_rate": 0.03,
    "annual_storage_cost_usd_per_logical_byte": 0.00001,
    "physical_replication_equivalent": 12,
    "legacy_entitlement_ckb_proxy": 45_000_000_000,
    "security_budget_stress_usd_y": [1_000_000, 10_000_000, 100_000_000],
    "note": "All resource/security quantities are explicit stress assumptions, not measured chain costs or forecasts."
}

def pv_rent(annual: float, years: int | None, rate: float) -> float:
    if years is None:
        return math.inf if rate <= 0 else annual / rate
    return sum(annual / ((1 + rate) ** y) for y in range(1, years + 1))

def row(price, duration, demand_name, users, hardware_name, hw_multiplier, path_name, terminal_multiplier):
    years = duration
    annual_physical = (ASSUMPTIONS["standard_output_bytes"] *
                       ASSUMPTIONS["annual_storage_cost_usd_per_logical_byte"] *
                       ASSUMPTIONS["physical_replication_equivalent"] * hw_multiplier)
    rent_total = annual_physical * years if years is not None else math.inf
    rent_pv = pv_rent(annual_physical, years, ASSUMPTIONS["annual_real_discount_rate"])
    current_extra = 60.0
    # A 61x conversion makes the 61-byte standard shape cost 1 CKB of capacity.
    # The 61x growth multiplier is a direct consequence, not a free parameter.
    models = [
        ("current_1ckb_per_byte", 61.0, 0.0, 0.0, 61),
        ("conversion_61_bytes_per_ckb", 1.0, 0.0, 0.0, 61),
        ("term_rent_expiring", 1.0, 0.0, rent_total, 61),
        ("prepaid_resource_endowment", 1.0, 0.0, rent_pv, 61),
        # Stress pricing: 1 CKB/byte base plus a 10x scarcity multiplier at
        # 10% spare credits. This is deliberately a stress rule, not equilibrium.
        ("market_credit_10pct_headroom", 1.0, 610.0, 0.0, 61),
    ]
    out=[]
    for name, cap_ckb, other_cap, rent_usd, state_bytes in models:
        extra = max(0.0, cap_ckb - 1.0) + other_cap
        nominal_rent = rent_usd
        annual_state_cost = annual_physical * users
        out.append({
            "model": name, "ckb_price_usd": price,
            "price_path": path_name, "terminal_ckb_price_usd": price*terminal_multiplier,
            "duration_years": "indefinite" if years is None else years,
            "demand_case": demand_name, "users": users,
            "hardware_case": hardware_name, "hardware_multiplier": hw_multiplier,
            "principal_ckb": 1, "principal_usd": price,
            "additional_capacity_ckb": extra, "additional_capacity_usd": extra*price,
            "refundable_collateral_ckb": extra, "nonrefundable_fee_ckb": ASSUMPTIONS["fixture_fee_ckb"],
            "rent_nominal_usd": ("upfront_endowment" if name=="prepaid_resource_endowment" else nominal_rent),
            "upfront_endowment_usd": (rent_pv if name=="prepaid_resource_endowment" else 0.0),
            "rent_present_value_usd_at_3pct": rent_pv,
            "locked_total_ckb_including_principal": 1+extra,
            "opportunity_cost_extra_collateral_usd_simple": ("unbounded_or_not_comparable" if years is None and extra else extra*price*ASSUMPTIONS["annual_opportunity_rate"]*(years or 0)),
            "state_bytes_per_user": state_bytes,
            "users_per_same_capacity_ckb_multiplier": (61 if name=="conversion_61_bytes_per_ckb" else 1),
            "aggregate_live_state_gb_decimal": state_bytes*users/1e9,
            "annual_resource_cost_usd_assumed": annual_state_cost,
            "annual_security_budget_stress_usd": ASSUMPTIONS["security_budget_stress_usd_y"],
            "security_cost_coverage_from_1e4_ckb_fee_usd": users*ASSUMPTIONS["fixture_fee_ckb"]*price*terminal_multiplier,
            "legacy_entitlement_proxy_ckb": ASSUMPTIONS["legacy_entitlement_ckb_proxy"],
            "legacy_current_entitlement_bytes_proxy": ASSUMPTIONS["legacy_entitlement_ckb_proxy"],
            "legacy_full_conversion_capacity_bytes_proxy": ASSUMPTIONS["legacy_entitlement_ckb_proxy"]*61,
            "legacy_redeem_split_status": "not_preserved_by_model_without_contractual_reserve",
        })
    return out

def clean(x):
    if isinstance(x,float) and not math.isfinite(x): return "infinite"
    if isinstance(x,list): return [clean(i) for i in x]
    if isinstance(x,dict): return {k:clean(v) for k,v in x.items()}
    return x

def main():
    rows=[]
    for p in PRICES:
      for d in DURATIONS:
       for dn,n in DEMANDS:
        for hn,hm in HARDWARE:
         for pn,pm in PRICE_PATHS:
          rows.extend(row(p,d,dn,n,hn,hm,pn,pm))
    OUT.mkdir(parents=True,exist_ok=True)
    with (OUT/'model-results.csv').open('w',newline='') as f:
        w=csv.DictWriter(f,fieldnames=list(rows[0]),lineterminator='\n'); w.writeheader()
        for x in rows: w.writerow(clean(x))
    (OUT/'model-results.json').write_text(json.dumps(clean({"assumptions":ASSUMPTIONS,"scenario_count":len(rows),"scenarios":rows}),indent=2,allow_nan=False)+'\n')
    liveness = {
      "case_offline_after_lease_expiry": {
        "delete_all_claim_state": {"claim_enforceable_after_expiry": False, "reason": "no consensus record from which the owner can prove amount, identity, and unspent status"},
        "retain_owner_receipt_only": {"claim_enforceable_after_expiry": False, "reason": "a signed receipt alone cannot establish that it was not already spent; needs consensus-visible replay/nullifier state and an authenticated anchor"},
        "retain_receipt_and_nullifier_index": {"claim_enforceable_after_expiry": True, "additional_persistent_state_required": True, "resource_rent_ends": False, "reason": "spend/replay state and data availability remain live until claim is consumed"},
        "renewal_censored_during_grace": {"claim_enforceable": False, "unless": "consensus keeps the claim live and accepts a proof/exit independent of the censored lease renewal"}
      },
      "conservation": "expiry may delete state only after all monetary rights are consumed or made unspendable; preserving rights requires persistent consensus-visible anti-replay state and available proof data"
    }
    (OUT/'lease-liveness.json').write_text(json.dumps(liveness,indent=2)+'\n')
    # Accounting checks: preserve principal/collateral distinction, state scaling,
    # price-crash scaling, and the two expiry outcomes that bound the claim.
    assert len(rows) == 1215
    current = next(x for x in rows if x['model']=='current_1ckb_per_byte' and x['ckb_price_usd']==10 and x['duration_years']==10 and x['demand_case']=='low' and x['hardware_case']=='flat' and x['price_path']=='stable')
    cheap = next(x for x in rows if x['model']=='conversion_61_bytes_per_ckb' and x['ckb_price_usd']==10 and x['duration_years']==10 and x['demand_case']=='low' and x['hardware_case']=='flat' and x['price_path']=='stable')
    crash = next(x for x in rows if x['model']=='current_1ckb_per_byte' and x['ckb_price_usd']==10 and x['duration_years']==10 and x['demand_case']=='low' and x['hardware_case']=='flat' and x['price_path']=='crash_90pct')
    assert current['principal_ckb']==1 and current['additional_capacity_ckb']==60 and current['locked_total_ckb_including_principal']==61
    assert cheap['additional_capacity_ckb']==0 and cheap['state_bytes_per_user']==61
    assert crash['security_cost_coverage_from_1e4_ckb_fee_usd']==current['security_cost_coverage_from_1e4_ckb_fee_usd']*0.1
    assert liveness['case_offline_after_lease_expiry']['delete_all_claim_state']['claim_enforceable_after_expiry'] is False
    assert liveness['case_offline_after_lease_expiry']['retain_receipt_and_nullifier_index']['additional_persistent_state_required'] is True
    print(f"wrote {len(rows)} rows: {OUT/'model-results.csv'} and model-results.json; liveness cases in lease-liveness.json")
    print(f"current 1y extra collateral: {60*PRICES[0]} USD at $0.01, {60*PRICES[1]} at $1, {60*PRICES[2]} at $10")
    print("conversion model: 61x more bytes per unit capacity; no demonstrated state/security budget neutrality")

if __name__ == '__main__': main()
