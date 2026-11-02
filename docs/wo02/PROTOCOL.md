# WO02 — Minimal type-enforced experiment

This is a bounded fixed-claims protocol, not a wallet, general payment system,
consensus fork or novel accumulator. It repairs WO01's admission gap without
changing WO01. Base sources and core validator versions remain pinned at WO01's
verified revision; the official SDK ACP binary is pinned separately in
[`sources.json`](../../evidence/wo02/sources.json).

## Representation and trust boundary

A live pool has:

- Empty-argument always-success lock: 33 occupied bytes.
- Type script: 33 bytes plus a 32-byte instance argument.
- Data: `depth_u8 || root_hash[32] || liabilities_u64_le`, 41 bytes.
- Capacity field: 8 bytes.

Total occupied capacity is **147 CKB**. The type authorizes every consumption,
including a transaction trying to strip it. Thus the empty-argument lock alone
is not the security boundary. A bare always-success cell is not a protected pool.
Clients must authenticate the type instance and state, not recognize a root by
its appearance. The lock is the exact 344-byte pinned CKB fixture, whose code
capital is included; no arbitrary insecure lock is called self-custodial.

All capacities/sums are u64 shannons. A leaf is:

```
leaf_hash = H(0 || ed25519_public_key[32] || amount_u64_le)
leaf_sum  = amount
parent_hash = H(1 || left_hash || left_sum || right_hash || right_sum)
parent_sum  = checked_add(left_sum, right_sum)
empty_leaf = (H(2), 0)
```

H is CKB-personalized 32-byte Blake2b. Zero entries must use a zero public key and
are canonical empty leaves. Positive entries are immutable claims. The tree is
padded to a power of two. Maximum depth is **8**, at most 256 slots. One admission
with **256 independently derived keys** was executed, not merely extrapolated.

The prototype deliberately reserves **147 additional CKB** beside total
liabilities: `cell.capacity = liabilities + 147 CKB`. This reserve keeps the root
funded for every remaining claim order. It is a conservative full-root reserve,
not a mathematical minimum for every chosen claim set: if every positive claim
were constrained to at least 1 CKB, 146 CKB could suffice until the last claim.
This version permits smaller claims and does not optimize reserve allocation.
Such a saving would not change the 61 CKB fresh standard-output boundary.
The reserve is irrevocably donated to the participants and the **last claimant
receives it at closure**. It is not an operator's separately reclaimable asset;
that policy and its last-exit incentive are disclosed rather than hidden as a
free subsidy.

The script fixes an exit fee of 10,000 shannons. Gross claim reduction is the
full claim; net payout subtracts the fee. No unadvertised operator key, recovery
interface, configurable fee oracle or dynamic deposits/payments are implemented.

## Admission

The single output at index zero must carry the executing type. The type argument
must equal `H(serialized_first_input || output_index_u64_le)`. There is one funding
input. This binds the instance to a unique consumable funding outpoint. Copying
the type to a different funding transaction fails; recreating it from the spent
original funding input also needs to pass the real outpoint resolver. This is an
explicit type-enforced creation rule, not a name or assumed operator registry.

The funding input's standard lock uses WitnessArgs.lock for its signature. The
type reads WitnessArgs.input_type, which contains:

```
0 || (public_key[32] || amount_u64_le) repeated 2^depth times
```

The type reconstructs the entire initial tree with checked arithmetic and compares
both hash and sum with the output. The exact capacity must equal sum + reserve.
Consequently, a claimed total cannot simply be chosen beside an opaque root.
The complete admission manifest is public witness data. This small full-manifest
construction is intentional; it makes the soundness/availability question
executable. It does not pretend to admit a million hidden accounts in one root
or implement scalable incremental deposits.

## Withdrawal and inductive invariant

Pool input is absolute input zero. There is exactly one input/output in the
executing type group, except that a fully closed pool has no successor. For an
exit the input_type payload is:

```
mode_u8 || index_u32 || amount_u64 || public_key[32]
        || (sibling_hash[32] || sibling_sum_u64) repeated depth
        || signature[64]
```

Modes are fresh output or an existing recipient cell. The type verifies the old
membership hash **and sum**, rejects overflow, and strictly checks the claimant's
Ed25519 signature of the raw transaction hash. That hash binds inputs, successor,
recipient, amounts and dependencies. The proof and key are authenticated by the
committed leaf/root; witness presence by itself conveys no authority.

Replacing the claimed leaf with the canonical empty leaf must produce:

```
new_liabilities = old_liabilities - claimed_amount
successor.capacity = new_liabilities + reserve
```

The successor type instance, depth and pool lock must be unchanged. Payout must
be exactly the gross claim minus fee. For an existing recipient, its input lock
must reappear with its full previous capacity plus that payout; extra inputs,
outputs, types/data and redirected recipient capital are rejected. Its own lock
also runs. WO02 uses the **actual ACP binary**, not an approximation of that
check. The last exit must leave the canonical empty root, retire the pool type,
and pay the remaining claim plus donated reserve minus the fee.

These rules preserve all other committed claims and their native backing by
induction from a validated admission. Root sums, reserves and monetary principal
are counted once. Membership proofs alone were insufficient in WO01; this
experiment checks admission and both sides of every transition. This is a
reasoned invariant backed by test cases, not a formal proof or production audit.

## Public reconstruction after operator failure

Each client retains its own authenticated original leaf and sibling hashes/sums.
A confirmed exit publishes the spent index and path. At the divergence level
`floor(log2(own_index XOR spent_index))`, the client replaces one cached sibling
with the changed subtree recomputed from the public path. Sums update with hashes.
No global account database, operator signing key or private state API is needed
for this fixed withdrawal-only transition.

The standalone Python client consumes only retained user state and actual packed
public CKB transactions. It decodes their Molecule fields, verifies outpoint
continuity, membership/sums, Ed25519 signatures, type identity, successor state,
and an independently authenticated expected current outpoint. A separate local
builder then constructs/signs the remaining user's exit and actual CKB validators
accept it. Public fixtures and attack results are retained under
[`operator-gone/`](../../evidence/wo02/operator-gone/).

This is conditional self-custody under **operator** disappearance with authentic
initial state, credentials, retrievable public history/code and transaction
inclusion. The archive can be served by an independent provider and need not be
trusted for its contents under these checks and anchors. The experiment supplies
anchors from its validated in-memory ledger, not a production light client or
independent live archive. It does not establish perpetual archive availability,
bounded miner censorship, safety under reorgs, or reconstruction after every
copy of the needed history/proof is lost. The client is not a complete CKB block
or witness-inclusion verifier.

## Deliberate limits

There are no general payments, key rotation/recovery, batching, cross-pool
coordination, fee-sponsor integration or UI. A small fresh output can fail even
when its claim is sound and its principal is fully backed. An existing ACP route
requires the recipient's pre-existing cell and capital. Dynamic payment utility,
production data availability and competition with Fiber/CoTA have not been
inferred from this simpler protocol. See [the decision](DECISION.md).
