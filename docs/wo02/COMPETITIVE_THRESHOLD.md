# WO02 — Competitive threshold (set before implementation)

Base: WO01 `ac79f830c896b8b431de9ce869df0883ffc416d5`. Existing WO01 source,
results and rejected hypotheses remain unchanged. References inherit exact pins
from `evidence/sources.json` and the WO01 source index.

The target is a **1 CKB claim** when CKB costs $1 or $10, under operator
failure. Separate these outcomes before comparing costs:

1. A fresh independently spendable standard L1 cell.
2. A credit into the user's already funded ACP cell.
3. A claim remaining in shared/channel state with a conditional exit route.

The comparison must hold initial solvency, credential authority, public data
availability, code availability, transaction inclusion assumptions and exit
representation constant. An empty root is not cheaper if its capital or data is
omitted; an existing recipient cell is not free infrastructure.

| Baseline | Equivalent useful property | Capital already required | What WO02 must outperform |
|---|---|---|---|
| Existing 1-of-2 multisig | Direct L1 ownership and precommitted alternative signer | >=61 CKB per standard-shaped cell; system code/data | Lower total native capital with equally direct fresh L1 control; changing to shared claims is not equivalent |
| ACP | Noninteractive credit into existing recipient cell, independently controlled by recipient afterward | >=61 CKB recipient capacity, code/data; thresholds may add bytes | A 1 CKB deposit alone needs only 1 CKB incremental principal plus fee; pool cannot claim this feature as new |
| CoTA | Existing authenticated aggregation and public-witness/syncer architecture | SDK per-lock 150 CKB allocation plus registry, assets, code, backing | Better total cost under same independent-user and operator-loss model; removing CoTA's distinct token/extension features is not superiority |
| Reusable shared UTXO / authenticated tree | Many signed rights per live cell | Root/lock/type, funding, code, authentic initial proofs and public deltas | Stronger admission/withdrawal invariant or demonstrably lower complete cost than the same reusable construction; a Merkle-sum tree is an existing technique |
| Fiber / payment channels | Repeated small shared/off-chain payments without a new L1 cell per payment | Funding/liquidity, settlement/reserve/code and latest-state monitoring requirements | Equal off-chain/shared or exit representation, offline/dispute/DA assumptions, measured funding and exit costs; not “root versus whole channel” |
| WO01 pool | Fixed authenticated claims; user updates cached proof from public exits | 73-byte root, native backing, code; conditional funding assumption | Reject underfunded admission and preserve funded liabilities; retain independent proof reconstruction without operator, including actual small-claim exit routes |

WO02 will not claim to beat Fiber or CoTA end-to-end without executing equivalent
workflows. Their unmeasured features/costs are explicit comparison limits, not
proof that the new pool wins. The first experiment is a falsification gate,
not authorization to expand a wallet or general payment protocol.

**Advance threshold:** valid adversarially tested admission/withdrawals plus a
materially better all-in capital or control outcome than the strongest reusable
alternative under the **same** exit and failure assumptions. Merely repairing
WO01's missing admission check does not meet that threshold.

**Stop expanding the pool** if the 1 CKB fresh-output objective still needs 61 CKB
per independently controlled recipient, while ACP and continued shared settlement
offer only already available economics. Evaluate protocol pricing separately;
this does not establish impossibility for all conceivable shared-state systems.
